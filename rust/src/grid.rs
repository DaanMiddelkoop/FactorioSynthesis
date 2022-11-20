use std::{collections::HashSet, cmp::{min, max}};

use crate::{entity::{Entity, Blueprint, self}, rotation::Rotation, building::Building, position::Position, recipes::{Recipe, State}, bounds::Bounds, astar::Astar};


#[derive(Clone)]
pub struct Grid {
    entities: Vec<Entity>,
    reserved: HashSet<Position>,
    output: Option<Position>,
    inputs: Vec<Position>,
    childs: Vec<Grid>,
    recipe: Recipe,
    bounds: Bounds,
}

impl Grid {
    pub fn new(recipe: Recipe) -> Self {
        Grid {
            entities: Vec::new(),
            reserved: HashSet::new(),
            output: None,
            inputs: Vec::new(),
            childs: Vec::new(),
            recipe,
            bounds: Bounds::new(),
        }
    }

    pub fn set_output(&mut self, output: Position) {
        self.output = Some(output);
    }

    pub fn add_input(&mut self, input: Position) {
        self.inputs.push(input);
    }

    fn move_absolute(&mut self, x: isize, y: isize) {
        let current_min_x = self.bounds.min_x;
        let current_min_y = self.bounds.min_y;

        self.move_relative(x - current_min_x, y - current_min_y);
    }

    fn move_relative(&mut self, x: isize, y: isize) {
        for entity in &mut self.entities {
            entity.position.x += x;
            entity.position.y += y;
        }

        let mut new_reserved = HashSet::new();
        for r in &self.reserved {
            let mut new_pos = r.clone();
            new_pos.x += x;
            new_pos.y += y;
            new_reserved.insert(new_pos);
        }
        self.reserved = new_reserved;

        if let Some(mut output) = self.output.clone() {
            output.x += x;
            output.y += y;
            self.output = Some(output);
        }

        for input in &mut self.inputs {
            input.x += x;
            input.y += y;
        }

        for child_grid in &mut self.childs {
            child_grid.move_relative(x, y);
        }

        self.bounds.min_x += x;
        self.bounds.max_x += x;
        self.bounds.min_y += y;
        self.bounds.max_y += y;

        
    }

    fn extend_grid(&mut self, other: Grid) {
        self.childs.push(other);
        // for entity in &other.entities {
        //     self.add_entity(entity.clone());
        // }
    }

    fn full_bounding_box(&self) -> Bounds {
        let mut b = self.bounds;
        for child_grid in &self.childs {
            b = b.combine(child_grid.full_bounding_box());
        }
        b
    }

    fn individual_boxes(&self) -> Vec<Bounds> {
        let mut result = Vec::new();
        result.push(self.bounds);
        for c in &self.childs {
            result.append(&mut c.individual_boxes());
        }
        result
    }


    pub fn add(&mut self, mut other: Grid, index: usize) {
        // Try to fit this grid anywhere and route to it. // Currently used a very very crude way of placing grids.
        let original_bounding_box = self.full_bounding_box();
        let mut other_bb = other.full_bounding_box();
        other_bb.min_x -= 2; // Compensate for routing space.
        let individual_boxes = self.individual_boxes();

        let mut best_position = (2, original_bounding_box.max_y);
        let mut best_area = 99999999;
        for x in 1..original_bounding_box.max_x + 2{
            for y in 0..original_bounding_box.max_y + 2 {
                let mut b = other_bb.clone();
                b.min_x += x;
                b.max_x += x;
                b.min_y += y;
                b.max_y += y;

                let mut collided = false;
                for individual_b in &individual_boxes {
                    
                    if b.collide(individual_b.clone()) {
                        collided = true;
                        break;
                    }
                }
                if collided {
                    continue;
                }


                let new_area = b.combine(original_bounding_box).area();
                if new_area < best_area {
                    best_area = new_area;
                    best_position = (x, y);
                }

            }
        }
        other.move_absolute(best_position.0, best_position.1);
        self.extend_grid(other.clone());
        self.route(other.output.unwrap(), self.inputs[index].clone(), other.recipe.output_state())
    }

    pub fn add_entity(&mut self, mut other: Entity) {
        other.entity_number = self.entities.len();

        let (w, h) = other.building.size();
        let (l, t) = other.building.origin();

        for x in 0..w {
            for y in 0..h {
                let pos_x = other.position.x;
                let pos_y = other.position.y;
                self.reserved.insert(Position {
                    x: pos_x + x - l,
                    y: pos_y + y - t,
                    rotation: Rotation::North,
                });
            }
        }

        self.bounds = self.bounds.combine(other.bounds());
        self.entities.push(other);
    }


    pub fn place_belts(&mut self, height: isize, x_offset: isize, rotation: Rotation, amount: isize) {
        let mut entity = Entity::from(Building::Belt);
        
        for i in 0..amount {
            entity.direction = rotation.assemble();
            entity.position.x = i + x_offset;
            entity.position.y = height;
            self.add_entity(entity.clone());
        }
    }

    pub fn place_poles(&mut self, height: isize, x_offset: isize, rotation: Rotation, amount: isize, skip: isize) {
        let entity = Entity::from(Building::Pole);
        self.place_entity(entity, height, x_offset, rotation, amount, skip);
    }

    pub fn place_inserters(&mut self, height: isize, x_offset: isize, rotation: Rotation, amount: isize, skip: isize) {
        let entity = Entity::from(Building::Inserter);
        self.place_entity(entity, height, x_offset, rotation, amount, skip);
    }

    pub fn place_long_inserters(&mut self, height: isize, x_offset: isize, rotation: Rotation, amount: isize, skip: isize) {
        let entity = Entity::from(Building::LongInserter);
        self.place_entity(entity, height, x_offset, rotation, amount, skip);
    }

    pub fn place_entity(&mut self, mut entity: Entity, height: isize, x_offset: isize, rotation: Rotation, amount: isize, skip: isize) {
        for i in 0..amount {
            entity.direction = rotation.assemble();
            entity.position.x = i * skip + x_offset;
            entity.position.y = height;
            self.add_entity(entity.clone());
        }
    }

    pub fn place_belt(&mut self, x: isize, y: isize, rotation: Rotation) {
        let mut entity = Entity::from(Building::Belt);
        entity.direction = rotation.assemble();
        entity.position.x = x;
        entity.position.y = y;
        self.add_entity(entity);
    }

    pub fn place_pipe_tunnel(&mut self, height: isize, x_offset: isize, rotation: Rotation, amount: isize, skip: isize) {
        let entity = Entity::from(Building::PipeTunnel);
        self.place_entity(entity, height, x_offset, rotation, amount, skip);
    }

    pub fn place_pipe(&mut self, height: isize, x_offset: isize, rotation: Rotation, amount: isize, skip: isize) {
        let entity = Entity::from(Building::Pipe);
        self.place_entity(entity, height, x_offset, rotation, amount, skip);
    }

    pub fn collect_entities(&self) -> Vec<Entity> {
        let mut entities = self.entities.clone();
        for child in &self.childs {
            let mut child_entities = child.collect_entities();
            entities.append(&mut child_entities);
        }
        entities
    }

    pub fn to_blueprint(&self) -> String {
        Blueprint::new(self.collect_entities()).serialize()
    }

    pub fn reserved_tiles(&self) -> HashSet<Position> {
        let mut tiles = self.reserved.clone();
        for c in &self.childs {
            tiles.extend(c.reserved_tiles());
        }
        tiles
    }

    pub fn route(&mut self, start: Position, end: Position, state: State) {
        let astar = Astar::new(start, end, self.reserved_tiles());


        if let Some(path) = astar.astar_belt() {
            for p in &path {
                let mut entity = Entity::from(Building::Belt);
                entity.position.x = p.x;
                entity.position.y = p.y;
                entity.direction = p.rotation.assemble();
                self.add_entity(entity);
            }
        } else {
            println!("Failed blueprint part: {}", self.to_blueprint());
            panic!()
        }
    }


}