#[repr(i32)]
#[derive(Debug)]
pub enum TeamID {
    Spectator = 1,
    T = 2,
    CT = 3,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

unsafe impl dataview::Pod for Vec3 {}

pub enum PlayerType {
    #[default]
    Unknown,
    Spectator,
    Local,
    Enemy,
    Team,
}

pub struct Player {
    pos: Vec3,
    yaw: f32,
    player_type: PlayerType,
    bomb: Bool,
    awp: Bool,
    scoped: Bool,
    player_name: String,
    weapon_id: i16,
    money: i32,
    health: u32,
}

impl Player {
    fn new(
        pos: Vec3,
        yaw: f32,
        player_type: PlayerType,
        bomb: Bool,
        awp: Bool,
        scoped: Bool,
        player_name: String,
        weapon_id: i16,
        money: i32,
        health: u32,
    ) -> Player {
        Player {
            pos,
            yaw,
            player_type,
            bomb,
            awp,
            scoped,
            player_name,
            weapon_id,
            money,
            health,
        }
    }
}
