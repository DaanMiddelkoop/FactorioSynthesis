use std::{collections::HashMap, hash::Hash};

use crate::building::Building;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum Recipe {
    IronOre,
    CopperOre,
    Stone,
    Coal,
    IronPlate,
    CopperPlate,
    FastInserter,
    Inserter,
    ElectronicCircuit,
    SmallElectricMotor,
    BurnerInserter,
    IronStick,
    SingleCylinderEngine,
    IronGearWheel,
    CopperCable,
    StoneTablet,
    StoneBrick,
    BigElectricMotor,
    Lubricant,
    SteelPlate,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum State {
    Solid,
    Liquid,
}

impl State {
    pub fn is_solid(&self) -> bool {
        return *self == State::Solid
    }

    pub fn is_liquid(&self) -> bool {
        return *self == State::Liquid
    }
}

impl Recipe {
    pub fn dependencies(&self) -> HashMap<Recipe, f64> {
        match self {
            Self::IronOre => HashMap::new(),
            Self::CopperOre => HashMap::new(),
            Self::Stone => HashMap::new(),
            Self::Coal => HashMap::new(),
            Self::IronPlate => HashMap::from([(Recipe::IronOre, 1.0)]),
            Self::CopperPlate => HashMap::from([(Recipe::CopperOre, 1.0)]),
            Self::FastInserter => HashMap::from([(Recipe::ElectronicCircuit, 2.0), (Recipe::Inserter, 1.0), (Recipe::IronPlate, 2.0)]),
            Self::Inserter => HashMap::from([(Recipe::SmallElectricMotor, 1.0), (Recipe::BurnerInserter, 1.0)]),
            Self::ElectronicCircuit => HashMap::from([(Recipe::CopperCable, 3.0), (Recipe::StoneTablet, 1.0)]),
            Self::SmallElectricMotor => HashMap::from([(Recipe::IronGearWheel, 1.0), (Recipe::CopperCable, 6.0), (Recipe::IronPlate, 1.0)]),
            Self::BurnerInserter => HashMap::from([(Recipe::IronStick, 2.0), (Recipe::SingleCylinderEngine, 1.0)]),
            Self::IronStick => HashMap::from([(Recipe::IronPlate, 0.5)]),
            Self::SingleCylinderEngine => HashMap::from([(Recipe::IronGearWheel, 1.0), (Recipe::IronPlate, 1.0)]),
            Self::IronGearWheel => HashMap::from([(Recipe::IronPlate, 2.0)]),
            Self::CopperCable => HashMap::from([(Recipe::CopperPlate, 0.5)]),
            Self::StoneTablet => HashMap::from([(Recipe::StoneBrick, 0.25)]),
            Self::StoneBrick => HashMap::from([(Recipe::Stone, 2.0)]),
            Self::BigElectricMotor => HashMap::from([(Recipe::Lubricant, 40.0), (Recipe::ElectronicCircuit, 4.0), (Recipe::SmallElectricMotor, 2.0), (Recipe::SteelPlate, 2.0)]),
            Self::SteelPlate => HashMap::from([(Recipe::IronPlate, 5.0)]),
            Self::Lubricant => HashMap::new(),
        }
    }

    pub fn crafting_time(&self) -> f64 {
        match self {
            Self::IronOre => 1.0,
            Self::CopperOre => 1.0,
            Self::Stone => 1.0,
            Self::Coal => 1.0,
            Self::IronPlate => 3.2,
            Self::CopperPlate => 3.2,
            Self::FastInserter => 0.5,
            Self::Inserter => 0.5,
            Self::ElectronicCircuit => 0.5,
            Self::SmallElectricMotor => 0.8,
            Self::BurnerInserter => 0.5,
            Self::IronStick => 0.25,
            Self::SingleCylinderEngine => 0.6,
            Self::IronGearWheel => 0.5,
            Self::CopperCable => 0.25,
            Self::StoneTablet => 0.125,
            Self::StoneBrick => 3.2,
            Self::BigElectricMotor => 10.0,
            Self::SteelPlate => 16.0,
            Self::Lubricant => 1.0,
        }
    }

    pub fn building(&self) -> Building {
        match self {
            Self::IronOre => Building::Miner,
            Self::CopperOre => Building::Miner,
            Self::Stone => Building::Miner,
            Self::Coal => Building::Miner,
            Self::IronPlate => Building::Furnace,
            Self::CopperPlate => Building::Furnace,
            Self::FastInserter => Building::Assembler(String::from("fast-inserter")),
            Self::Inserter => Building::Assembler(String::from("inserter")),
            Self::ElectronicCircuit => Building::Assembler(String::from("electronic-circuit")),
            Self::SmallElectricMotor => Building::Assembler(String::from("electric-motor")),
            Self::BurnerInserter => Building::Assembler(String::from("burner-inserter")),
            Self::IronStick => Building::Assembler(String::from("iron-stick")),
            Self::SingleCylinderEngine => Building::Assembler(String::from("motor")),
            Self::IronGearWheel => Building::Assembler(String::from("iron-gear-wheel")),
            Self::CopperCable => Building::Assembler(String::from("copper-cable")),
            Self::StoneTablet => Building::Assembler(String::from("stone-tablet")),
            Self::StoneBrick => Building::Assembler(String::from("stone-brick")),
            Self::BigElectricMotor => Building::Assembler(String::from("electric-engine-unit")),
            Self::SteelPlate => Building::Furnace,
            Self::Lubricant => Building::ChemicalLab,
        }
    }

    pub fn output_state(&self) -> State {
        match self {
            Self::IronOre => State::Solid,
            Self::CopperOre => State::Solid,
            Self::Stone => State::Solid,
            Self::Coal => State::Solid,
            Self::IronPlate => State::Solid,
            Self::CopperPlate => State::Solid,
            Self::FastInserter => State::Solid,
            Self::Inserter => State::Solid,
            Self::ElectronicCircuit => State::Solid,
            Self::SmallElectricMotor => State::Solid,
            Self::BurnerInserter => State::Solid,
            Self::IronStick => State::Solid,
            Self::SingleCylinderEngine => State::Solid,
            Self::IronGearWheel => State::Solid,
            Self::CopperCable => State::Solid,
            Self::StoneTablet => State::Solid,
            Self::StoneBrick => State::Solid,
            Self::BigElectricMotor => State::Solid,
            Self::SteelPlate => State::Solid,
            Self::Lubricant => State::Liquid,
        }
    }
    
}