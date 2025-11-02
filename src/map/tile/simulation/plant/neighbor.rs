/// Describes the direction of a neighbor
#[derive(Clone, Debug)]
pub enum NeighborType {
    Right,
    UpRight,
    UpLeft,
    Left,
    DownLeft,
    DownRight,
}

impl NeighborType {
    /// A unique id for the neighbor direction which serves as a priority for acting on a neighbor
    pub fn id(&self) -> usize {
        return match self {
            Self::Right => 2,
            Self::UpRight => 4,
            Self::UpLeft => 5,
            Self::Left => 3,
            Self::DownLeft => 1,
            Self::DownRight => 0,
        };
    }
}
