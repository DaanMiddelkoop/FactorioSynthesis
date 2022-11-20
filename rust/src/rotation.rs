
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Rotation {
    North,
    East,
    South,
    West,
}

impl Rotation {
    pub fn assemble(&self) -> usize {
        match self {
            Self::North => 4,
            Self::East => 2,
            Self::South => 0,
            Self::West => 6,
        }
    }
}