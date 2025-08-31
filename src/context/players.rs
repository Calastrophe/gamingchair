use crate::offsets::{
    client::CCSPlayerController::m_hPlayerPawn,
    client_dll::{dwLocalPlayerController, dwLocalPlayerPawn},
};

use super::offsets;
use super::{equipment::Equipment, pointers::Pointers};
use memflow::prelude::v1::*;
use player::Player;
use relation::Relation;
use teamid::TeamID;
use vec3::Vec3;

mod player;
pub mod relation;
pub mod teamid;
mod vec3;

#[derive(Default, Debug)]
pub struct Players {
    local_player: Player,
    other_players: Vec<Player>,
}

impl Players {
    pub fn sides(&self) -> (Vec<&Player>, Vec<&Player>) {
        self.iter()
            .partition(|player| player.team_id == self.local_player.team_id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Player> {
        std::iter::once(&self.local_player).chain(self.other_players.iter())
    }

    pub fn update(&mut self, process: &mut IntoProcessInstanceArcBox<'static>, ptrs: &Pointers) {
        // Fetch local player
        let mut local_controller = 0u64;
        let mut local_pawn = 0u64;

        {
            let mut batcher = process.batcher();

            batcher.read_into(
                ptrs.module_base + dwLocalPlayerController,
                &mut local_controller,
            );
            batcher.read_into(ptrs.module_base + dwLocalPlayerPawn, &mut local_pawn);
        }

        let ent_list = Address::from(ptrs.entity_list);
        self.local_player = Player::read(
            process,
            local_controller.into(),
            local_pawn.into(),
            ent_list,
        );
        self.local_player.relation = Relation::Local;

        // Fetch other players
        let mut list_entries = [0u64; 64];

        {
            let mut batcher = process.batcher();
            list_entries.iter_mut().enumerate().for_each(|(idx, data)| {
                batcher.read_into(ent_list + ((8 * (idx & 0x7FFF) >> 9) + 16), data);
            });
        }

        let mut controllers = [0u64; 64];

        {
            let mut batcher = process.batcher();
            controllers.iter_mut().enumerate().for_each(|(idx, data)| {
                let list_entry: Address = list_entries[idx].into();
                batcher.read_into(list_entry + 120 * (idx & 0x1FF), data);
            });
        }

        let mut pawn_handles = [0u64; 64];

        {
            let mut batcher = process.batcher();
            pawn_handles.iter_mut().enumerate().for_each(|(idx, data)| {
                let entity_controller: Address = controllers[idx].into();
                batcher.read_into(entity_controller + m_hPlayerPawn, data);
            })
        }

        let mut entity_list = [0u64; 64];

        {
            let mut batcher = process.batcher();
            entity_list.iter_mut().enumerate().for_each(|(idx, data)| {
                batcher.read_into(
                    ent_list + (8 * ((pawn_handles[idx] & 0x7FFF) >> 9) + 16),
                    data,
                );
            })
        }

        let mut pawns = [0u64; 64];

        {
            let mut batcher = process.batcher();
            pawns.iter_mut().enumerate().for_each(|(idx, data)| {
                let list_entry: Address = entity_list[idx].into();
                batcher.read_into(list_entry + (120) * (pawn_handles[idx] & 0x1FF), data);
            })
        }

        self.other_players = controllers
            .iter()
            .zip(pawns.iter())
            .filter_map(|(controller, pawn)| {
                let controller_addr = Address::from(*controller);
                let pawn_addr = Address::from(*pawn);

                if controller_addr.is_null() || pawn_addr.is_null() {
                    None
                } else {
                    Some((controller_addr, pawn_addr))
                }
            })
            .filter_map(|(controller, pawn)| {
                if controller == self.local_player.controller {
                    None
                } else {
                    Some(Player::read(process, controller, pawn, ent_list))
                }
            })
            .map(|mut player| {
                player.relation = match player.team_id {
                    TeamID::Unknown => Relation::Unknown,
                    TeamID::Spectator => Relation::Spectator,
                    _ if self.local_player.team_id == player.team_id => Relation::Teammate,
                    _ => Relation::Enemy,
                };
                player
            })
            .collect();
    }
}
