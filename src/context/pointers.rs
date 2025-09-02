use super::offsets;
use memflow::prelude::v1::*;

#[derive(Default, Clone, Copy)]
pub struct Pointers {
    pub module_base: Address,
    pub global_vars: Address,
    pub entity_list: Address,
    pub game_rules: Address,
    pub game_entity_system: Address,
}

impl Pointers {
    pub fn new(module_base: Address) -> Pointers {
        Pointers {
            module_base,
            ..Default::default()
        }
    }

    pub fn update(&mut self, process: &mut IntoProcessInstanceArcBox<'static>) {
        let mut global_vars = 0u64;
        let mut game_rules = 0u64;
        let mut entity_list = 0u64;
        let mut game_entity_system = 0u64;

        {
            let mut batcher = process.batcher();
            batcher.read_into(
                self.module_base + offsets::client_dll::dwGlobalVars,
                &mut global_vars,
            );
            batcher.read_into(
                self.module_base + offsets::client_dll::dwGameRules,
                &mut game_rules,
            );
            batcher.read_into(
                self.module_base + offsets::client_dll::dwEntityList,
                &mut entity_list,
            );
            batcher.read_into(
                self.module_base + offsets::client_dll::dwGameEntitySystem,
                &mut game_entity_system,
            );
        }

        self.global_vars = Address::from(global_vars);
        self.game_rules = Address::from(game_rules);
        self.entity_list = Address::from(entity_list);
        self.game_entity_system = Address::from(game_entity_system);
    }
}
