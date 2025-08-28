pub mod maps;

use super::Pointers;
use maps::Map;
use memflow::prelude::v1::*;

const CURRENT_MAP_OFFSET: u64 = 0x180;

#[derive(Default)]
pub struct Information {
    pub current_map: Map,
}

impl Information {
    pub fn in_game(&self) -> bool {
        self.current_map != Map::Empty
    }

    pub fn update(&mut self, process: &mut IntoProcessInstanceArcBox<'static>, ptrs: &Pointers) {
        let mut map_ptr = 0u64;

        let map_address = Address::from(ptrs.global_vars + CURRENT_MAP_OFFSET);

        // TODO: Potential undefined behavior happening?
        let _ = process.read_into(map_address, &mut map_ptr);

        let map_name = process
            .read_utf8_lossy(map_ptr.into(), 32)
            .unwrap_or_default();

        self.current_map = Map::from(map_name.as_str());
    }
}
