use crate::offsets;
use information::Information;
use memflow::prelude::v1::*;
use players::Players;
use pointers::Pointers;

mod equipment;
mod information;
mod players;
mod pointers;

// TODO: Create a structure for information about other entities - the bomb, deployed smokes, and molotovs.
pub struct Context {
    process: IntoProcessInstanceArcBox<'static>,
    ptrs: Pointers,
    pub players: Players,
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
            players: Players::default(),
            information: Information::default(),
        }
    }

    pub fn update(&mut self) {
        self.ptrs.update(&mut self.process);
        self.information.update(&mut self.process, &self.ptrs);

        if self.information.in_game() {
            self.players.update(&mut self.process, &self.ptrs);
        }
    }
}
