use super::equipment::Equipment;
use crate::offsets::{
    client::CCSPlayerController::m_hPlayerPawn,
    client_dll::{dwLocalPlayerController, dwLocalPlayerPawn},
};
use memflow::prelude::v1::*;
use player::{Player, relation::Relation, teamid::TeamID};

pub mod player;

#[derive(Debug, Default)]
pub struct Players {
    pub local_player: Player,
    pub other_players: Vec<Player>,
}

impl Players {
    pub fn update(
        &mut self,
        process: &mut IntoProcessInstanceArcBox<'static>,
        module_base: Address,
        entity_list: Address,
    ) {
        let mut local_controller = 0u64;
        let mut local_pawn = 0u64;

        {
            let mut batcher = process.batcher();

            batcher.read_into(module_base + dwLocalPlayerController, &mut local_controller);
            batcher.read_into(module_base + dwLocalPlayerPawn, &mut local_pawn);
        }

        self.local_player = Player::read(
            process,
            local_controller.into(),
            local_pawn.into(),
            entity_list,
        );
        self.local_player.relation = Relation::Local;

        let mut list = [0u64; 64];

        {
            let mut batcher = process.batcher();
            list.iter_mut().enumerate().for_each(|(idx, entry)| {
                batcher.read_into(entity_list + ((8 * (idx & 0x7FFF) >> 9) + 16), entry);
            });
        }

        // Get the player controllers.
        let mut controllers = [0u64; 64];

        {
            let mut batcher = process.batcher();
            controllers
                .iter_mut()
                .enumerate()
                .for_each(|(idx, controller)| {
                    let list_entry: Address = list[idx].into();
                    batcher.read_into(list_entry + 120 * (idx & 0x1FF), controller);
                });
        }

        let controllers = controllers.map(|c| Address::from(c));

        // Get the handle to player pawns.
        let mut pawn_handles = [0u64; 64];

        {
            let mut batcher = process.batcher();
            pawn_handles
                .iter_mut()
                .enumerate()
                .for_each(|(idx, handle)| {
                    batcher.read_into(controllers[idx] + m_hPlayerPawn, handle);
                })
        }

        // Get the entity list entries based off the pawn handles.
        let mut entries = [0u64; 64];

        {
            let mut batcher = process.batcher();
            entries.iter_mut().enumerate().for_each(|(idx, entry)| {
                batcher.read_into(
                    entity_list + ((8 * (pawn_handles[idx] & 0x7FFF) >> 9) + 16),
                    entry,
                );
            });
        }

        // Retrieve the actual pawns.
        let mut pawns = [0u64; 64];

        {
            let mut batcher = process.batcher();
            pawns.iter_mut().enumerate().for_each(|(idx, pawn)| {
                let list_entry: Address = entries[idx].into();
                batcher.read_into(list_entry + 120 * (pawn_handles[idx] & 0x1FF), pawn);
            });
        }

        let pawns = pawns.map(|p| Address::from(p));

        self.other_players = controllers
            .iter()
            .zip(pawns.iter())
            .filter_map(|(controller, pawn)| {
                if controller.is_null()
                    || pawn.is_null()
                    || *controller == self.local_player.controller
                {
                    None
                } else {
                    Some(Player::read(process, *controller, *pawn, entity_list))
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
