#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Side {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
}
impl Side {
    pub(crate) fn name(&self) -> &'static str {
        match self {
            Side::Top => "top",
            Side::Bottom => "bottom",
            Side::Left => "left",
            Side::Right => "right",
            Side::Front => "front",
            Side::Back => "back",
        }
    }
}
