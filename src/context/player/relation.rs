#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Relation {
    #[default]
    Unknown,
    Spectator,
    Local,
    Enemy,
    Teammate,
}
