use super::pointers::Pointers;
use memflow::prelude::v1::*;

use players::{Players, player::Player};

pub mod equipment;
pub mod players;

#[derive(Default, Debug)]
pub struct Entities {
    players: Players,
    // TODO: projectiles
    // TODO: weapons
}

impl Entities {
    pub fn players(&self) -> impl Iterator<Item = &Player> {
        std::iter::once(&self.players.local_player).chain(self.players.other_players.iter())
    }

    pub fn sides(&self) -> (Vec<&Player>, Vec<&Player>) {
        self.players()
            .partition(|player| player.team_id == self.players.local_player.team_id)
    }

    pub fn update(&mut self, process: &mut IntoProcessInstanceArcBox<'static>, ptrs: &Pointers) {
        self.players
            .update(process, ptrs.module_base, ptrs.entity_list);
    }
}
