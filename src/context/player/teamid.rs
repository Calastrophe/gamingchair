#[derive(Debug, Default, Eq, PartialEq)]
pub enum TeamID {
    #[default]
    Unknown,
    Spectator,
    T,
    CT,
}

impl From<i32> for TeamID {
    fn from(value: i32) -> Self {
        match value {
            1 => TeamID::Spectator,
            2 => TeamID::T,
            3 => TeamID::CT,
            _ => TeamID::Unknown,
        }
    }
}
