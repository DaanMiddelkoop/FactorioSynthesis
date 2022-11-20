use crate::position::Position;



#[derive(Clone, PartialEq, Eq)]
pub enum Building {
    Assembler(String),
    Furnace,
    ChemicalLab,
    Belt,
    Pole,
    Inserter,
    LongInserter,
    Miner,
    Pipe,
    PipeTunnel,
}

impl Building {
    pub fn size(&self) -> (isize, isize) {
        match self {
            Building::Assembler(_) => (3, 3),
            Building::Furnace => (3, 3),
            Building::Belt => (1, 1),
            Building::Pole => (1, 1),
            Building::Inserter => (1, 1),
            Building::LongInserter => (1, 1),
            Building::Miner => (3, 3),
            Building::ChemicalLab => (3, 3),
            Building::Pipe => (1, 1),
            Building::PipeTunnel => (1, 1),
        }
    }

    pub fn origin(&self) -> (isize, isize) {
        match self {
            Self::Assembler(_) => (1, 1),
            Self::Furnace => (1, 1),
            Self::Belt => (0, 0),
            Self::Pole => (0, 0),
            Self::Inserter => (0, 0),
            Self::LongInserter => (0, 0),
            Self::Miner => (1, 1),
            Self::ChemicalLab => (1, 1),
            Self::Pipe => (0, 0),
            Self::PipeTunnel => (0, 0),
        }
    }
}

pub enum Input {
    Belt(Position),
    Pipe(Position),
    Building(Building),
}

