use std::time::Instant;

use memflow::prelude::v1::*;

use crate::offsets;

use super::Pointers;

#[derive(Default, Debug)]
pub struct Bomb {
    pub is_planted: bool,
    pub is_defusing: bool,
    pub fuse_length: f32,
    pub fuse_stamp: Option<Instant>,
    pub defuse_length: f32,
    pub defuse_stamp: Option<Instant>,
}

impl Bomb {
    pub fn update(&mut self, process: &mut IntoProcessInstanceArcBox<'static>, ptrs: &Pointers) {
        let mut is_planted = 0u8;

        let _ = process.read_into(
            ptrs.game_rules + offsets::client::C_CSGameRules::m_bBombPlanted,
            &mut is_planted,
        );

        self.is_planted = is_planted != 0;

        self.fuse_stamp = self
            .is_planted
            .then(|| self.fuse_stamp.unwrap_or_else(Instant::now));

        // Early return if the bomb is not planted.
        if !self.is_planted {
            return;
        }

        // Read relevant information about the bomb and if its being defused.
        let bomb_ptr = process
            .read_addr64(ptrs.module_base + offsets::client_dll::dwPlantedC4)
            .unwrap();
        let bomb = process.read_addr64(bomb_ptr).unwrap();

        let mut is_defusing = 0u8;

        {
            let mut batcher = process.batcher();

            batcher.read_into(
                bomb + offsets::client::C_PlantedC4::m_bBeingDefused,
                &mut is_defusing,
            );
            batcher.read_into(
                bomb + offsets::client::C_PlantedC4::m_flTimerLength,
                &mut self.fuse_length,
            );
            batcher.read_into(
                bomb + offsets::client::C_PlantedC4::m_flDefuseLength,
                &mut self.defuse_length,
            );
        }

        self.is_defusing = is_defusing != 0;

        self.defuse_stamp = self
            .is_defusing
            .then(|| self.defuse_stamp.unwrap_or_else(Instant::now));
    }
}
