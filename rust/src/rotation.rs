
#[derive(PartialEq, PartialOrd, Eq, Hash, Clone, Copy, Debug)]
pub enum Rotation {
    North,
    East,
    South,
    West,
}

impl Rotation {
    pub fn assemble(&self) -> usize {
        match self {
            Self::North => 0,
            Self::East => 2,
            Self::South => 4,
            Self::West => 6,
        }
    }
}