use crate::rotation::Rotation;



#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Position {
    pub x: isize,
    pub y: isize,
    pub rotation: Rotation,
}

impl Position {
    pub fn new(x: isize, y: isize, rotation: Rotation) -> Self {
        Position {
            x,
            y,
            rotation,
        }
    }

    pub fn north(x: isize, y: isize) -> Self {
        Position {
            x,
            y,
            rotation: Rotation::North,
        }
    }

    pub fn move_rel(&self, x: isize, y: isize) -> Self {
        Position {
            x: self.x + x,
            y: self.y + y,
            rotation: self.rotation
        }
    }

    pub fn rotate_left(&self) -> Self {
        Position {
            x: self.x,
            y: self.y,
            rotation: match self.rotation {
                Rotation::North => Rotation::West,
                Rotation::East => Rotation::North,
                Rotation::South => Rotation::East,
                Rotation::West => Rotation::South
            }
        }
    }

    pub fn rotate_right(&self) -> Self {
        Position {
            x: self.x,
            y: self.y,
            rotation: match self.rotation {
                Rotation::North => Rotation::East,
                Rotation::East => Rotation::South,
                Rotation::South => Rotation::West,
                Rotation::West => Rotation::North
            }
        }
    }

    pub fn forward(&self) -> Self {
        match self.rotation {
            Rotation::North => self.move_rel(0, -1),
            Rotation::East => self.move_rel(1, 0),
            Rotation::South => self.move_rel(0, 1),
            Rotation::West => self.move_rel(-1, 0)
        }
    }

    pub fn backward(&self) -> Self {
        match self.rotation {
            Rotation::North => self.move_rel(0, 1),
            Rotation::East => self.move_rel(-1, 0),
            Rotation::South => self.move_rel(0, -1),
            Rotation::West => self.move_rel(1, 0)
        }
    }
}