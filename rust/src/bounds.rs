use std::cmp::{min, max};





#[derive(Copy, Clone, Debug)]
pub struct Bounds {
    pub min_x: isize,
    pub max_x: isize,
    pub min_y: isize, 
    pub max_y: isize,
}

impl Bounds {
    pub fn new() -> Self {
        Bounds {
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
        }
    }

    pub fn combine(&self, other: Bounds) -> Self {
        Bounds {
            min_x: min(self.min_x, other.min_x),
            max_x: max(self.max_x, other.max_x),
            min_y: min(self.min_y, other.min_y),
            max_y: max(self.max_y, other.max_y)
        }
    }

    pub fn area(&self) -> isize {
        (self.max_x - self.min_x) * (self.max_y - self.min_y)
    }

    pub fn collide_part(&self, other: Bounds) -> bool {
        let a = other.min_x < self.min_x && self.min_x < other.max_x;
        let b = other.min_x < self.max_x && self.max_x < other.max_x;
        let c = other.min_y < self.min_y && self.min_y < other.max_y;
        let d = other.min_y < self.max_y && self.max_y < other.max_y;

        (a && c) || (a && d) || (b && c) || (b && d)
    }

    pub fn collide(&self, other: Bounds) -> bool {
        self.min_x < other.min_x + (other.max_x - other.min_x) &&
        self.min_x + (self.max_x - self.min_x) > other.min_x && 
        self.min_y < other.min_y + (other.max_y - other.min_y) &&
        self.min_y + (self.max_y - self.min_y) > other.min_y

    }
}