use std::{collections::{HashMap, BTreeMap}, hash::Hash};

use rand::Rng;

use crate::{recipes::Recipe, position::Position, building::Building, rotation::Rotation, grid::Grid, entity::Entity};



#[derive(PartialEq, PartialOrd, Debug)]
struct FloatVec {
    pub x: f64,
    pub y: f64,
}

impl FloatVec {
    fn len(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }

    fn sub(&self, other: &FloatVec) -> FloatVec {
        FloatVec { x: self.x- other.x, y: self.y - other.y }
    }

    fn add(&self, other: &FloatVec) -> FloatVec {
        FloatVec { x: other.x + self.x, y: other.y + self.y }
    }

    fn mul(&self, factor: f64) -> FloatVec {
        FloatVec { x: self.x * factor, y: self.y * factor }
    }
}


#[derive(PartialEq)]
struct Node {
    pub pos: FloatVec,
    pub building: Building,
}

pub struct SpringSystem {
    nodes: Vec<Node>,
    dependencies: BTreeMap<usize, (usize, f64)>,
    recipe: Recipe,
    velocities: Vec<FloatVec>,
}

impl SpringSystem {
    pub fn new(recipe: Recipe, item_per_sec: f64) -> Self {

        let mut system = Self {
            nodes: Vec::new(),
            dependencies: BTreeMap::new(),
            recipe,
            velocities: Vec::new(),
        };

        let center_node = Node {
            pos: FloatVec { x: 0.0, y: 0.0 },
            building: Building::Belt,
        };

        system.nodes.push(center_node);
        system.velocities.push(FloatVec { x: 0.0, y: 0.0 });
        

        system.generate(recipe, item_per_sec, Some(0), 1.0);
        for count in 0..1000 {
            system.cycle(count);
        }
        system
    }

    fn generate(&mut self, recipe: Recipe, item_per_sec: f64, parent: Option<usize>, layer: f64) {
        if recipe.building() == Building::Miner {
            return;
        }

        println!("Creating {:?} amount: {}", recipe, item_per_sec);

        let mut rng = rand::thread_rng();
        let float_devices_needed = recipe.crafting_time() * item_per_sec;

        let devices_needed = float_devices_needed.ceil() as usize;
        let parts_per_device = float_devices_needed / float_devices_needed.ceil();
        for _ in 0..devices_needed {
            let node = Node {
                pos: FloatVec { x: rng.gen(), y: rng.gen() },
                building: recipe.building()
            };
            println!("Initial positions: {}, {:?}", self.nodes.len(), node.pos);

            self.nodes.push(node);
            self.velocities.push(FloatVec { x: 0.0, y: 0.0 });
            let node_index = self.nodes.len() - 1;

            println!("Parent: {:?}", parent);
            if let Some(p) = parent {
                self.dependencies.insert(node_index, (p, 1.0 / layer));
            }

            // generate children for node.
            for (c, amount) in recipe.dependencies() {
                println!("Some index: {:?}", Some(node_index));
                self.generate(c, parts_per_device * amount, Some(node_index), layer * 2.0);
            }
        }
    }

    fn cycle(&mut self, count: usize) {
        let mut forces: HashMap<usize, FloatVec> = HashMap::new();
        for index in 0..self.nodes.len() {
            forces.insert(index, FloatVec { x: 0.0, y: 0.0 });
        }


        for (i1, n1) in self.nodes.iter().enumerate() {
            for (i2, n2) in self.nodes.iter().enumerate() {
                if i1 == i2 { continue;}
                if n1.pos.x > 1e20 || n2.pos.x > 1e20 {
                    panic!("Numbers getting too large")
                }
                
                let diff = n1.pos.sub(&n2.pos);
                let dist = diff.len() * diff.len();
                let mut repulsion = 1.0;
                if diff.len() > 10.0 {
                    repulsion = 0.0;
                }


                let force = FloatVec { x: diff.x * (repulsion / dist), y: diff.y * (repulsion / dist) };
                // println!("Repulsion between {i1}: {:?} and {i2}: {:?} is {:?}", n1.pos, n2.pos, force);
                
                let previous = forces.remove(&i1).unwrap();
                forces.insert(i1, previous.add(&force));
            }
        }

        for (i1, (i2, force)) in &self.dependencies {
            let n1 = &self.nodes[*i1];
            let n2 = &self.nodes[*i2];
            let mut diff = n2.pos.sub(&n1.pos).mul(0.1);
            let mut dist = diff.len() * force;

            diff = diff.mul(dist * dist * dist);
            let prev1 = forces.remove(&i1).unwrap();
            let prev2 = forces.remove(&i2).unwrap();
            

            forces.insert(*i1, prev1.add(&diff));
            forces.insert(*i2, prev2.sub(&diff));

            // println!("Attaction between : {i1}, {i2}, forces: {:?}, n1.x: {}, n2.x: {}", diff, n1.pos.x, n2.pos.x);
        }

        println!("Velocities: {:?}", self.velocities);
        // Apply forces to nodes.
        for (index, node) in self.nodes.iter_mut().enumerate() {
            let force = forces.remove(&index).unwrap();
            // println!("Node: {index}, forces: {:?} position: {:?}", force, node.pos);

            self.velocities[index] = self.velocities[index].add(&force.mul(0.01)).mul(0.99);
            node.pos = node.pos.add(&self.velocities[index]);
        }
    }

    pub fn partial_blueprint(&self) {
        let mut grid = Grid::new(self.recipe);
        for node in &self.nodes {
            println!("Dependencies: {:?}", self.dependencies);
            println!("Node position: {:?}", node.pos);
            let mut entity = Entity::from(node.building.clone());
            entity.position.x = (node.pos.x) as isize;
            entity.position.y = (node.pos.y) as isize;
            entity.direction = Rotation::North.assemble();
            grid.add_entity(entity);
        }
        println!("Partial blueprint: {}", grid.to_blueprint());
    }
}