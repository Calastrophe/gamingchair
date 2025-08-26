use information::Information;
use memflow::prelude::v1::*;
use players::Players;
use pointers::Pointers;

use crate::offsets::{
    self,
    client_dll::{dwLocalPlayerController, dwLocalPlayerPawn},
};

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

        Context {
            process,
            ptrs: Pointers::new(client_module.base),
            players: Players::default(),
            information: Information::default(),
        }
    }

    pub fn update(&mut self) {
        self.ptrs.update(&mut self.process);
        self.players.update(&mut self.process, &self.ptrs);
        self.information.update(&mut self.process, &self.ptrs);
    }
}
