use memflow::os::Process as MProcess;
use memflow::prelude::v1::*;

use crate::offsets::offsets_mod::cs2_dumper::offsets::client_dll::{dwEntityList, dwGameEntitySystem, dwGameRules, dwGlobalVars, dwLocalPlayerController, dwLocalPlayerPawn};
use crate::offsets::offsets_mod::cs2_dumper::offsets::engine2_dll::dwBuildNumber;

const PROCESS_NAME: &str = if cfg!(target_os = "windows") {
    "cs2.exe"
} else {
    "cs2"
};
const CLIENT_MODULE: &str = if cfg!(target_os = "windows") {
    "client.dll"
} else {
    "client.so"
};

const ENGINE_MODULE: &str = if cfg!(target_os = "windows") {
    "engine2.dll"
} else {
    "engine2.so"
};

pub struct Context {
    process: IntoProcessInstanceArcBox<'static>,
    client_module: ModuleInfo,
    engine_module: ModuleInfo,
}

impl Context {
    pub fn new(os: OsInstanceArcBox<'static>) -> Self {
        let mut process = os.into_process_by_name(PROCESS_NAME).unwrap();

        let client_module = process.module_by_name(CLIENT_MODULE).unwrap();
        log::info!("{CLIENT_MODULE} @ {:010x}", client_module.address);
        let engine_module = process.module_by_name(ENGINE_MODULE).unwrap();
        log::info!("{ENGINE_MODULE} @ {:010x}", engine_module.address);

        Context {
            process,
            client_module,
            engine_module,
        }
    }
    //TODO; refactor this
    pub fn read_all_refactor_later(&mut self)  { 
        //get all ptrs for important stuff
        let dw_global_vars: usize = self.process.read(self.client_module.base + dwGlobalVars).unwrap();
        let dw_game_rules: usize  = self.process.read(self.client_module.base + dwGameRules).unwrap();
        let dw_entity_list: usize  = self.process.read(self.client_module.base + dwEntityList).unwrap();
        let dw_game_entity_system: usize  = self.process.read(self.client_module.base + dwGameEntitySystem).unwrap();

        let dw_local_player_controller: usize = self.process.read(self.client_module.base + dwLocalPlayerController).unwrap();
        let dw_local_player_pawn: usize = self.process.read(self.engine_module.base + dwLocalPlayerPawn).unwrap();





    }
    

}
