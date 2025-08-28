use super::offsets;
use super::{Equipment, Relation, TeamID, Vec3, util};
use egui::{Align2, Color32, FontId, Painter, Pos2, Rect, Stroke, Ui, Vec2};
use memflow::prelude::v1::*;

const SIGHT_LINE_LENGTH: f32 = 14.0;
const SCOPED_LINE_LENGTH: f32 = 1000.0;
const CIRCLE_RADIUS: f32 = 8.0;
const MAX_HEALTH: f32 = 100.0;
const FONT_SIZE: f32 = 9.0;
const NAME_OFFSET: Vec2 = Vec2::new(0.0, 20.0);
const CAUTION_OFFSET: Vec2 = Vec2::new(0.0, -25.0);
const UTILITY_OFFSET: Vec2 = Vec2::new(0.0, -22.0);
const UTILITY_SIZE: Vec2 = Vec2::new(12.0, 20.0);

#[derive(Default, Debug)]
pub struct Player {
    pub controller: Address,
    pub pawn: Address,
    pub health: u32,
    pub position: Vec3,
    pub yaw: f32,
    pub team_id: TeamID,
    pub relation: Relation,
    pub name: String,
    pub money: i32,
    pub current_weapon: Equipment,
    pub loadout: Vec<Equipment>,
    pub is_scoped: bool,
}

impl Player {
    pub fn new(controller: Address, pawn: Address) -> Player {
        Player {
            controller,
            pawn,
            ..Default::default()
        }
    }

    fn is_enemy(&self) -> bool {
        matches!(self.relation, Relation::Enemy)
    }

    fn color(&self) -> Color32 {
        match self.relation {
            Relation::Unknown | Relation::Spectator => Color32::GRAY,
            Relation::Enemy => Color32::RED,
            Relation::Teammate => Color32::BLUE,
            Relation::Local => Color32::GREEN,
        }
    }

    pub fn draw(
        &self,
        ui: &Ui,
        painter: &Painter,
        (offset_x, offset_y): (f32, f32),
        scale: f32,
        image_scaling: f32,
    ) {
        // Convert the player's coordinates into map coordinates then scaled by the current image scaling.
        let position = Pos2::new(
            ((self.position.x - offset_x) / scale) * image_scaling,
            ((self.position.y - offset_y) / -scale) * image_scaling,
        );

        // Create a vector in the direction they are looking.
        let yaw = self.yaw.to_radians();
        let line_length = if self.is_scoped {
            SCOPED_LINE_LENGTH
        } else {
            SIGHT_LINE_LENGTH
        };
        let direction_vector = Vec2::new(line_length * yaw.cos(), -line_length * yaw.sin());

        // Draw a straight line if they are scoped, otherwise an arrow.
        if self.is_scoped {
            let end = position + direction_vector;
            painter.line_segment([position, end], Stroke::new(1.0, Color32::WHITE));
        } else {
            painter.arrow(position, direction_vector, Stroke::new(2.0, Color32::WHITE));
        }

        // Draw a circle to indicate player's position.
        let radius = CIRCLE_RADIUS * image_scaling;
        painter.circle(position, radius, self.color(), Stroke::NONE);

        let health_percentage = self.health as f32 / MAX_HEALTH;
        let start_angle = -std::f32::consts::FRAC_PI_2;
        let end_angle = start_angle + 2.0 * std::f32::consts::PI * health_percentage;

        // Draw a ring around the player's circle indicating remaining health.
        util::generate_arc(
            painter,
            position,
            radius,
            start_angle,
            end_angle,
            Stroke::new(1.5, Color32::WHITE),
        );

        // Display player's name a little below their circle.
        painter.text(
            position + NAME_OFFSET,
            Align2::CENTER_CENTER,
            &self.name,
            FontId::monospace(FONT_SIZE),
            Color32::WHITE,
        );

        // Caution opposing enemy players with snipers!
        if self.is_enemy()
            && matches!(
                self.current_weapon,
                Equipment::AWP | Equipment::SSG08 | Equipment::SCAR20
            )
        {
            painter.text(
                position + CAUTION_OFFSET,
                Align2::CENTER_CENTER,
                "!",
                FontId::monospace(FONT_SIZE * 2.0),
                Color32::ORANGE,
            );
        }

        // Display any utility being held by all players.
        if matches!(
            self.current_weapon,
            Equipment::Flashbang
                | Equipment::Decoy
                | Equipment::Molotov
                | Equipment::HE
                | Equipment::Smoke
                | Equipment::Incendiary
        ) {
            let image = self.current_weapon.image().unwrap();
            let rect = Rect::from_center_size(position + UTILITY_OFFSET, UTILITY_SIZE);

            image.paint_at(ui, rect);
        }
    }

    pub fn read(
        process: &mut IntoProcessInstanceArcBox<'static>,
        controller: Address,
        pawn: Address,
        entity_list: Address,
    ) -> Player {
        let mut player = Player::new(controller, pawn);
        let mut team = 0i32;
        let mut is_scoped = 0u8;
        let mut clipping_weapon_ptr = 0u64;
        let mut money_services_ptr = 0u64;
        let mut weapon_services_ptr = 0u64;
        let mut player_name_ptr = 0u64;

        {
            let mut batcher = process.batcher();

            batcher.read_into(
                pawn + offsets::client::C_BasePlayerPawn::m_vOldOrigin,
                &mut player.position,
            );
            batcher.read_into(
                pawn + offsets::client::C_CSPlayerPawn::m_angEyeAngles + 4,
                &mut player.yaw,
            );
            batcher.read_into(
                pawn + offsets::client::C_BaseEntity::m_iHealth,
                &mut player.health,
            );
            batcher.read_into(
                pawn + offsets::client::C_CSPlayerPawn::m_bIsScoped,
                &mut is_scoped,
            );
            batcher.read_into(
                pawn + offsets::client::C_CSPlayerPawn::m_pClippingWeapon,
                &mut clipping_weapon_ptr,
            );
            batcher.read_into(
                pawn + offsets::client::C_BasePlayerPawn::m_pWeaponServices,
                &mut weapon_services_ptr,
            );
            batcher.read_into(
                controller + offsets::client::CCSPlayerController::m_pInGameMoneyServices,
                &mut money_services_ptr,
            );
            batcher.read_into(
                controller + offsets::client::CCSPlayerController::m_sSanitizedPlayerName,
                &mut player_name_ptr,
            );
            batcher.read_into(
                controller + offsets::client::C_BaseEntity::m_iTeamNum,
                &mut team,
            );
        }

        let (
            money_services_address,
            player_name_address,
            clipping_weapon_address,
            weapon_services_address,
        ) = (
            Address::from(money_services_ptr),
            Address::from(player_name_ptr),
            Address::from(clipping_weapon_ptr),
            Address::from(weapon_services_ptr),
        );

        player.money = process
            .read(
                money_services_address
                    + offsets::client::CCSPlayerController_InGameMoneyServices::m_iAccount,
            )
            .unwrap_or_default();

        player.name = process
            .read_utf8_lossy(player_name_address, 32)
            .unwrap_or_default();

        let item_idx_addr = clipping_weapon_address
            + offsets::client::C_EconEntity::m_AttributeManager
            + offsets::client::C_AttributeContainer::m_Item
            + offsets::client::C_EconItemView::m_iItemDefinitionIndex;

        let item_idx: i16 = process.read(item_idx_addr).unwrap_or_default();

        player.current_weapon = Equipment::from(item_idx);

        let mut weapon_count = 0i32;
        let mut weapons_base_ptr = 0u64;

        {
            let mut batcher = process.batcher();
            batcher.read_into(
                weapon_services_address + offsets::client::CPlayer_WeaponServices::m_hMyWeapons,
                &mut weapon_count,
            );
            batcher.read_into(
                weapon_services_address
                    + offsets::client::CPlayer_WeaponServices::m_hMyWeapons
                    + 0x8,
                &mut weapons_base_ptr,
            );
        }

        let weapon_count = weapon_count.clamp(0, 32) as usize;
        let weapons_base_address = Address::from(weapons_base_ptr);
        let mut weapon_handles = vec![0u64; weapon_count];

        {
            let mut batcher = process.batcher();
            weapon_handles
                .iter_mut()
                .enumerate()
                .for_each(|(idx, handle)| {
                    batcher.read_into(weapons_base_address + (idx * 0x4), handle);
                });
        }

        let mut list_entries = vec![0u64; weapon_count];

        {
            let mut batcher = process.batcher();
            list_entries
                .iter_mut()
                .zip(&weapon_handles)
                .for_each(|(list_entry, weapon_handle)| {
                    batcher.read_into(
                        entity_list + 0x8 * ((weapon_handle & 0x7FFF) >> 9) + 16,
                        list_entry,
                    );
                });
        }

        {
            let mut batcher = process.batcher();
            list_entries
                .iter_mut()
                .zip(&weapon_handles)
                .for_each(|(entry, weapon_handle)| {
                    let address = Address::from(*entry + 120 * (weapon_handle & 0x1FF));
                    batcher.read_into(address, entry);
                });
        }

        let weapons = list_entries;
        let mut weapon_idxs = vec![0i16; weapon_count];
        {
            let mut batcher = process.batcher();
            weapons
                .iter()
                .zip(&mut weapon_idxs)
                .for_each(|(weapon, weapon_idx)| {
                    let weapon_address = Address::from(*weapon);

                    let weapon_idx_address = weapon_address
                        + offsets::client::C_EconEntity::m_AttributeManager
                        + offsets::client::C_AttributeContainer::m_Item
                        + offsets::client::C_EconItemView::m_iItemDefinitionIndex;

                    batcher.read_into(weapon_idx_address, weapon_idx);
                })
        }

        player.loadout = weapon_idxs
            .iter()
            .filter_map(|weapon_idx| {
                let weapon = Equipment::from(*weapon_idx);

                if matches!(weapon, Equipment::Unknown | Equipment::Knife) {
                    None
                } else {
                    Some(weapon)
                }
            })
            .collect();

        player.is_scoped = is_scoped != 0;
        player.team_id = TeamID::from(team);
        player
    }
}
