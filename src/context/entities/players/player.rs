use super::Equipment;
use crate::offsets;
use egui::Color32;
use memflow::prelude::v1::*;
use relation::Relation;
use teamid::TeamID;
use vec3::Vec3;

pub mod relation;
pub mod teamid;
mod vec3;

#[derive(Default, Debug)]
pub struct Player {
    pub controller: Address,
    pub pawn: Address,
    pub health: u32,
    pub name: String,
    pub money: i32,
    pub team_id: TeamID,
    pub relation: Relation,
    pub position: Vec3,
    pub yaw: f32,
    pub current_weapon: Equipment,
    pub loadout: Vec<Equipment>,
    pub is_scoped: bool,
    pub is_bomb_carrier: bool,
}

impl Player {
    pub fn new(controller: Address, pawn: Address) -> Player {
        Player {
            controller,
            pawn,
            ..Default::default()
        }
    }

    pub fn is_enemy(&self) -> bool {
        matches!(self.relation, Relation::Enemy)
    }

    pub fn color(&self) -> Color32 {
        match self.relation {
            Relation::Unknown | Relation::Spectator => Color32::GRAY,
            Relation::Enemy => Color32::RED,
            Relation::Teammate => Color32::BLUE,
            Relation::Local => Color32::GREEN,
        }
    }

    fn read_current_weapon(
        &mut self,
        process: &mut IntoProcessInstanceArcBox<'static>,
        clipping_weapon: Address,
    ) {
        let weapon_idx_address = clipping_weapon
            + offsets::client::C_EconEntity::m_AttributeManager
            + offsets::client::C_AttributeContainer::m_Item
            + offsets::client::C_EconItemView::m_iItemDefinitionIndex;

        let item_idx: i16 = process.read(weapon_idx_address).unwrap_or_default();

        self.current_weapon = Equipment::from(item_idx);
    }

    fn read_loadout(
        &mut self,
        process: &mut IntoProcessInstanceArcBox<'static>,
        weapon_services: Address,
        entity_list: Address,
    ) {
        let mut weapon_count = 0i32;
        let mut weapons_ptr = 0u64;

        {
            let mut batcher = process.batcher();
            batcher.read_into(
                weapon_services + offsets::client::CPlayer_WeaponServices::m_hMyWeapons,
                &mut weapon_count,
            );
            batcher.read_into(
                weapon_services + offsets::client::CPlayer_WeaponServices::m_hMyWeapons + 0x8,
                &mut weapons_ptr,
            );
        }

        let weapon_count = weapon_count.clamp(0, 32) as usize;
        let weapons_address = Address::from(weapons_ptr);
        let mut weapon_handles = vec![0u64; weapon_count];

        // Iterate over each expected weapon and read their respective handle.
        {
            let mut batcher = process.batcher();
            weapon_handles
                .iter_mut()
                .enumerate()
                .for_each(|(idx, handle)| {
                    batcher.read_into(weapons_address + (idx * 0x4), handle);
                });
        }

        let mut list_entries = vec![0u64; weapon_count];

        // From the handles, read the respective list entries in the entity list.
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

        // From the entries and weapon handles, read the actual weapon entities.
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

        let mut weapon_idxs = vec![0i16; weapon_count];

        // From the weapon entities, read the item definition indices.
        {
            let mut batcher = process.batcher();
            list_entries
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

        // Convert the item definition indices into actual equipment variants.
        self.loadout = weapon_idxs
            .iter()
            .filter_map(|weapon_idx| {
                let weapon = Equipment::from(*weapon_idx);

                // If they are holding the bomb in their loadout, indicate that.
                if weapon == Equipment::C4 {
                    self.is_bomb_carrier = true;
                }

                (weapon != Equipment::Unknown).then(|| weapon)
            })
            .collect();

        // Sort the loadout by equipment categories.
        self.loadout.sort_by(|a, b| a.category().cmp(&b.category()));
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

        let (money_services, player_name, clipping_weapon, weapon_services) = (
            Address::from(money_services_ptr),
            Address::from(player_name_ptr),
            Address::from(clipping_weapon_ptr),
            Address::from(weapon_services_ptr),
        );

        // Read the player's current money.
        player.money = process
            .read(
                money_services
                    + offsets::client::CCSPlayerController_InGameMoneyServices::m_iAccount,
            )
            .unwrap_or_default();

        // Read the player's name.
        player.name = process.read_utf8_lossy(player_name, 32).unwrap_or_default();

        player.read_current_weapon(process, clipping_weapon);

        player.read_loadout(process, weapon_services, entity_list);

        player.is_scoped = is_scoped != 0;
        player.team_id = TeamID::from(team);
        player
    }
}
