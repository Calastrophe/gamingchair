use maps::Map;
use memflow::prelude::v1::*;
use player::{Player, relation::Relation, teamid::TeamID};

use crate::offsets::{
    self,
    client_dll::{dwEntityList, dwGlobalVars, dwLocalPlayerController, dwLocalPlayerPawn},
};

mod equipment;
mod maps;
pub mod player;
mod vec3;

pub const CURRENT_MAP: u64 = 0x180;

pub struct Context {
    process: IntoProcessInstanceArcBox<'static>,
    client_module: ModuleInfo,
    engine_module: ModuleInfo,

    globals: u64,
    entity_list: u64,

    pub local_player: Player,
    pub players: Vec<Player>,
    pub map: Map,
}

impl Context {
    pub fn new(os: OsInstanceArcBox<'static>) -> Self {
        let mut process = os.into_process_by_name("cs2.exe").unwrap();

        let client_module = process.module_by_name("client.dll").unwrap();

        let engine_module = process.module_by_name("engine2.dll").unwrap();

        Context {
            process,
            client_module,
            engine_module,
            globals: 0,
            entity_list: 0,
            local_player: Player::default(),
            players: Vec::new(),
            map: Map::Empty,
        }
    }

    pub fn players(&self) -> impl Iterator<Item = &Player> {
        self.players
            .iter()
            .chain(std::iter::once(&self.local_player))
    }

    pub fn update(&mut self) {
        self.update_pointers();
        self.update_locals();
        self.update_players();
    }

    pub fn update_pointers(&mut self) {
        let mut batcher = self.process.batcher();

        batcher.read_into(self.client_module.base + dwGlobalVars, &mut self.globals);
        batcher.read_into(
            self.client_module.base + dwEntityList,
            &mut self.entity_list,
        );
    }

    pub fn update_locals(&mut self) {
        let mut batcher = self.process.batcher();

        let mut local_controller = 0u64;
        let mut local_pawn = 0u64;
        let mut map_ptr = 0u64;

        batcher.read_into(
            self.client_module.base + dwLocalPlayerController,
            &mut local_controller,
        );
        batcher.read_into(self.client_module.base + dwLocalPlayerPawn, &mut local_pawn);

        let map_addr = (self.globals + CURRENT_MAP).into();
        batcher.read_into(map_addr, &mut map_ptr);

        drop(batcher);

        let map_string = self.process.read_utf8_lossy(map_ptr.into(), 32).ok();

        if let Some(map) = map_string {
            self.map = Map::from(map.as_str());
        }

        self.local_player = Player::read(
            &mut self.process,
            local_controller.into(),
            local_pawn.into(),
        );
        self.local_player.relation = Relation::Local;
    }

    pub fn update_players(&mut self) {
        let mut list_entries = [0u64; 64];
        let ent_list = Address::from(self.entity_list);

        {
            let mut batcher = self.process.batcher();
            list_entries
                .iter_mut()
                .enumerate()
                .skip(1)
                .for_each(|(idx, data)| {
                    batcher.read_into(ent_list + ((8 * (idx & 0x7FFF) >> 9) + 16), data);
                });
        }

        let mut controllers = [0u64; 64];

        {
            let mut batcher = self.process.batcher();
            controllers
                .iter_mut()
                .enumerate()
                .skip(1)
                .for_each(|(idx, data)| {
                    let list_entry: Address = list_entries[idx].into();
                    batcher.read_into(list_entry + 120 * (idx & 0x1FF), data);
                });
        }

        let mut pawns = [0u64; 64];

        {
            let mut batcher = self.process.batcher();
            pawns
                .iter_mut()
                .enumerate()
                .skip(1)
                .for_each(|(idx, data)| {
                    let entity_controller: Address = controllers[idx].into();
                    batcher.read_into(
                        entity_controller + offsets::client::CCSPlayerController::m_hPlayerPawn,
                        data,
                    );
                })
        }

        let mut entity_list = [0u64; 64];

        {
            let mut batcher = self.process.batcher();
            entity_list
                .iter_mut()
                .enumerate()
                .skip(1)
                .for_each(|(idx, data)| {
                    batcher.read_into(ent_list + (8 * ((pawns[idx] & 0x7FFF) >> 9) + 16), data);
                })
        }

        let mut actual_pawns = [0u64; 64];

        {
            let mut batcher = self.process.batcher();
            actual_pawns
                .iter_mut()
                .enumerate()
                .skip(1)
                .for_each(|(idx, data)| {
                    let list_entry: Address = entity_list[idx].into();
                    batcher.read_into(list_entry + (120) * (pawns[idx] & 0x1FF), data);
                })
        }

        self.players = controllers
            .iter()
            .zip(actual_pawns.iter())
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
                let player = Player::read(&mut self.process, controller, pawn);

                if player == self.local_player {
                    None
                } else {
                    Some(player)
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
