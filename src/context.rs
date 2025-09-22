use crate::offsets;
use entities::Entities;
use information::Information;
use memflow::prelude::v1::*;
use pointers::Pointers;

pub mod entities;
pub mod information;
mod pointers;

pub struct Context {
    process: IntoProcessInstanceArcBox<'static>,
    ptrs: Pointers,
    pub entities: Entities,
    pub information: Information,
}

impl Context {
    pub fn new(os: OsInstanceArcBox<'static>) -> Self {
        let mut process = os.into_process_by_name("cs2.exe").unwrap();
        let client_module = process.module_by_name("client.dll").unwrap();
        let engine_module = process.module_by_name("engine2.dll").unwrap();

        let game_build: u32 = process
            .read(engine_module.base + offsets::engine2_dll::dwBuildNumber)
            .unwrap();
        let supported_game_build = env!("CS2_BUILD_NUMBER").parse::<u32>().unwrap();

        if game_build != supported_game_build {
            panic!(
                "Unsupported game build: {game_build}, current supported game build: {supported_game_build}."
            )
        }

        Context {
            process,
            ptrs: Pointers::new(client_module.base),
            entities: Entities::default(),
            information: Information::default(),
        }
    }

    pub fn update(&mut self) {
        if self.process.state().is_dead() {
            return;
        }

        self.ptrs.update(&mut self.process);
        self.information.update(&mut self.process, &self.ptrs);

        if self.information.in_game() {
            self.entities.update(&mut self.process, &self.ptrs);
        }
    }
}
