use crate::offsets;

use super::{equipment::Equipment, vec3::Vec3};
use memflow::prelude::v1::*;
use relation::Relation;
use teamid::TeamID;

pub mod relation;
pub mod teamid;

#[derive(Default, Debug)]
pub struct Player {
    controller: Address,
    pawn: Address,
    pub health: u32,
    pub position: Vec3,
    pub yaw: f32,
    pub team_id: TeamID,
    pub relation: Relation,
    pub name: String,
    pub money: i32,
    pub current_weapon: Equipment,
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

    pub fn read(
        process: &mut IntoProcessInstanceArcBox<'static>,
        controller: Address,
        pawn: Address,
    ) -> Player {
        let mut player = Player::new(controller, pawn);
        let mut team = 0i32;
        let mut is_scoped = 0u8;
        let mut clipping_weapon_ptr = 0u64;
        let mut money_services_ptr = 0u64;
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

        let (money_services_address, player_name_address, clipping_weapon_address) = (
            Address::from(money_services_ptr),
            Address::from(player_name_ptr),
            Address::from(clipping_weapon_ptr),
        );

        if !money_services_address.is_null() {
            player.money = process
                .read(
                    money_services_address
                        + offsets::client::CCSPlayerController_InGameMoneyServices::m_iAccount,
                )
                .unwrap_or_default();
        }

        if !player_name_address.is_null() {
            player.name = process
                .read_utf8_lossy(player_name_address, 32)
                .unwrap_or_default();
        }

        if !clipping_weapon_address.is_null() {
            let item_idx_addr = clipping_weapon_address
                + offsets::client::C_EconEntity::m_AttributeManager
                + offsets::client::C_AttributeContainer::m_Item
                + offsets::client::C_EconItemView::m_iItemDefinitionIndex;

            let item_idx: i16 = process.read(item_idx_addr).unwrap_or_default();

            player.current_weapon = Equipment::from(item_idx);
        }

        player.is_scoped = is_scoped != 0;
        player.team_id = TeamID::from(team);
        player
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.controller == other.controller
    }

    fn ne(&self, other: &Self) -> bool {
        self.controller != other.controller
    }
}
