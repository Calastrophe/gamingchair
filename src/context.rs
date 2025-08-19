use maps::Map;
use memflow::prelude::v1::*;
use player::{Player, relation::Relation};

use crate::offsets::{
    self,
    client_dll::{dwEntityList, dwGlobalVars, dwLocalPlayerController, dwLocalPlayerPawn},
};

mod equipment;
mod maps;
mod player;
mod vec3;

pub const CURRENT_MAP: u64 = 0x1B8;

pub struct Context {
    process: IntoProcessInstanceArcBox<'static>,
    client_module: ModuleInfo,
    engine_module: ModuleInfo,

    globals: u64,
    entity_list: u64,

    local_player: Player,
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

        {
            let mut batcher = self.process.batcher();
            let ent_list = Address::from(self.entity_list);
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

        self.players = controllers
            .iter()
            .zip(pawns.iter())
            .map(|(controller, pawn)| (Address::from(*controller), Address::from(*pawn)))
            .filter(|(controller, pawn)| !controller.is_null() && !pawn.is_null())
            .map(|(controller, pawn)| Player::read(&mut self.process, controller, pawn))
            .filter(|player| *player != self.local_player)
            .collect();
    }
}
