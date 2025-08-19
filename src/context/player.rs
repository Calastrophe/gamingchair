use crate::offsets;

use super::vec3::Vec3;
use memflow::prelude::v1::*;
use relation::Relation;
use teamid::TeamID;

pub mod relation;
mod teamid;

#[derive(Default, Debug)]
pub struct Player {
    controller: Address,
    pawn: Address,
    pub health: u32,
    pub position: Vec3,
    pub yaw: f32,
    team_id: TeamID,
    pub relation: Relation,
}

impl Player {
    pub fn new(controller: Address, pawn: Address) -> Player {
        Player {
            controller,
            pawn,
            ..Default::default()
        }
    }

    pub fn read(
        process: &mut IntoProcessInstanceArcBox<'static>,
        controller: Address,
        pawn: Address,
    ) -> Player {
        let mut player = Player::new(controller, pawn);
        let mut team = 0i32;

        {
            let mut batcher = process.batcher();

            batcher.read_into(
                pawn + offsets::client::C_BasePlayerPawn::m_vOldOrigin,
                &mut player.position,
            );
            batcher.read_into(
                pawn + offsets::client::C_CSPlayerPawnBase::m_angEyeAngles + 4,
                &mut player.yaw,
            );
            batcher.read_into(
                pawn + offsets::client::C_BaseEntity::m_iHealth,
                &mut player.health,
            );
            batcher.read_into(
                controller + offsets::client::C_BaseEntity::m_iTeamNum,
                &mut team,
            );
        }

        player.team_id = TeamID::from(team);
        player
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.controller == other.controller
    }

    fn ne(&self, other: &Self) -> bool {
        self.controller != other.controller
    }
}
