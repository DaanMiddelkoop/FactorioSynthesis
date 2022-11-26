use std::io::Write;

use flate2::Compression;
use flate2::write::ZlibEncoder;
use serde::{Serialize, Deserialize};
use serde_json;

use crate::bounds::Bounds;
use crate::building::Building;
use crate::building::Building::*;
use crate::position::Position;
use crate::rotation::Rotation;

#[derive(Serialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct EntityPosition {
    pub x: isize,
    pub y: isize,
}

#[derive(Serialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Entity {
    pub entity_number: usize,
    pub name: String,
    pub position: EntityPosition,
    pub direction: usize,
    pub recipe: String,
    #[serde(skip_serializing)]
    pub building: Building,
}

impl Entity {

    pub fn from(building: Building) -> Self {
        match building {
            Assembler(recipe) => Entity {entity_number: 0, name: String::from("assembling-machine-1"), position: EntityPosition { x: 0, y: 0 }, direction: 0, recipe: recipe.clone(), building: Assembler(recipe)},
            Furnace => Entity { entity_number: 0, name: String::from("electric-furnace"), position: EntityPosition { x: 0, y: 0 }, direction: 0, recipe: String::new(), building: Furnace },
            Belt => Entity { entity_number: 0, name: String::from("fast-transport-belt"), position: EntityPosition { x: 0, y: 0 }, direction: 0, recipe: String::new(), building: Belt},
            Pole => Entity { entity_number: 0, name: String::from("small-iron-electric-pole"), position: EntityPosition { x: 0, y: 0 }, direction: 0, recipe: String::new(), building: Pole},
            Inserter => Entity { entity_number: 0, name: String::from("fast-inserter"), position: EntityPosition { x: 0, y: 0 }, direction: 0, recipe: String::new(), building: Inserter},
            LongInserter => Entity { entity_number: 0, name: String::from("long-handed-inserter"), position: EntityPosition { x: 0, y: 0 }, direction: 0, recipe: String::new(), building: LongInserter},
            Miner => Entity { entity_number: 0, name: String::from("miner"), position: EntityPosition { x: 0, y: 0 }, direction: 0, recipe: String::new(), building: Miner},
            ChemicalLab => Entity { entity_number: 0, name: String::from("chemical-lab"), position: EntityPosition { x: 0, y: 0 }, direction: 0, recipe: String::new(), building: ChemicalLab},
            Pipe => Entity { entity_number: 0, name: String::from("pipe"), position: EntityPosition { x: 0, y: 0 }, direction: 0, recipe: String::new(), building: Pipe},
            PipeTunnel => Entity { entity_number: 0, name: String::from("pipe-to-ground"), position: EntityPosition { x: 0, y: 0 }, direction: 0, recipe: String::new(), building: PipeTunnel},
        }
    }

    pub fn bounds(&self) -> Bounds {
        let (w, h) = self.building.size();
        let (l, t) = self.building.origin();

        Bounds {
            min_x: self.position.x - l,
            max_x: self.position.x - l + w,
            min_y: self.position.y - t,
            max_y: self.position.y - t + h,
        }
    }

    pub fn set_position(&mut self, mut position: Position) {
        match self.building {
            Building::Inserter => { 
                self.position.x = position.x;
                self.position.y = position.y;
                self.direction = position.rotate_left().rotate_left().rotation.assemble(); }

            _ => {
                self.position.x = position.x;
                self.position.y = position.y;
                self.direction = position.rotation.assemble();
            }
        }
    }
}

#[derive(Serialize)]
pub struct Blueprint {
    entities: Vec<Entity>,
    item: String,
    version: usize,
}

impl Blueprint {
    pub fn new(entities: Vec<Entity>) -> Self {
        Blueprint {
            entities, 
            item: String::from("blueprint"),
            version: 281479276199938,
        }
    }

    pub fn serialize(&self) -> String {
        let body = String::from("{\"blueprint\": ") + serde_json::to_string(self).unwrap().as_str() + "}";
        // println!("body: {}", body);
        let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
        e.write_all(body.as_bytes()).unwrap();
        let compressed = e.finish().unwrap();
        String::from("0") + base64::encode(compressed).as_str()
    }
}