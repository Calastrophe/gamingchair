use super::offsets;
use memflow::prelude::v1::*;

#[derive(Default, Clone, Copy)]
pub struct Pointers {
    pub module_base: Address,
    pub global_vars: u64,
    pub entity_list: u64,
    pub game_rules: u64,
    pub game_entity_system: u64,
}

impl Pointers {
    pub fn new(module_base: Address) -> Pointers {
        Pointers {
            module_base,
            ..Default::default()
        }
    }

    pub fn update(&mut self, process: &mut IntoProcessInstanceArcBox<'static>) {
        let mut batcher = process.batcher();
        batcher.read_into(
            self.module_base + offsets::client_dll::dwGlobalVars,
            &mut self.global_vars,
        );
        batcher.read_into(
            self.module_base + offsets::client_dll::dwGameRules,
            &mut self.game_rules,
        );
        batcher.read_into(
            self.module_base + offsets::client_dll::dwEntityList,
            &mut self.entity_list,
        );
        batcher.read_into(
            self.module_base + offsets::client_dll::dwGameEntitySystem,
            &mut self.game_entity_system,
        );
    }
}
