use std::collections::{HashMap, HashSet};

use crate::{position::Position, recipes::Recipe, building::{Building, self}, entity::{Entity, EntityPosition}, rotation::Rotation, astar::{self, Astar}};
use rand::seq::SliceRandom; // 0.7.2

#[derive(Clone)]
pub struct Grid {
    pub buildings: HashMap<Position, Building>,
    pub taken: HashSet<Position>,
    pub copper_ore: Vec<Position>,
    pub iron_ore: Vec<Position>,
}

impl Grid {
    pub fn to_blueprint(&self) -> String {
        let mut pub_grid = crate::grid::Grid::new(Recipe::IronOre);
        
        for (pos, building) in &self.buildings {
            let mut entity = Entity::from(building.clone());
            entity.set_position(pos.clone());
            pub_grid.add_entity(entity);
        }   

        pub_grid.to_blueprint()
    }
}

impl Grid {
    pub fn new(copper: Position, iron: Position) -> Self {
        

        let mut grid = Self {
            buildings: HashMap::new(),
            taken: HashSet::new(),
            copper_ore: vec![copper],
            iron_ore: vec![iron],
        };
        grid.add(Building::Belt, copper);
        grid.add(Building::Belt, iron);
        grid
    }

    pub fn add(&mut self, building: Building, pos: Position) {
        self.buildings.insert(pos, building.clone());
        let (w, h) = building.size();
        let (ox, oy) = building.origin();
        for x in 0..w {
            for y in 0..h {
                self.taken.insert(Position {x: pos.x + x - ox, y: pos.y + y - oy, rotation: Rotation::North});
            }
        }
    }

    pub fn remove(&mut self, pos: Position) {
        let building = self.buildings.remove(&pos).unwrap();
        let (w, h) = building.size();
        let (ox, oy) = building.origin();
        for x in 0..w {
            for y in 0..h {
                self.taken.remove(&Position {x: pos.x + x - ox, y: pos.y + y - oy, rotation: Rotation::North});
            }
        }
    }

    pub fn is_free(&self, building: Building, pos: Position) -> bool {
        let (w, h) = building.size();
        let (ox, oy) = building.origin();
        for x in 0..w {
            for y in 0..h {
                if self.taken.contains(&Position {x: pos.x + x - ox, y: pos.y + y - oy, rotation: Rotation::North}) {
                    return false
                }
            }
        }
        true
    }
}


#[derive(Debug)]
pub struct Possibility {
    heads: Vec<Node>,
}

#[derive(Clone, Debug)]
pub struct Node {
    recipe: Recipe,
    building: Building,
    pos: Position,
    amount_per_sec: f64,
}

pub fn generate_recipe(recipe: Recipe, amount_per_sec: f64) -> String {
    let mut grid = Grid::new(
        Position { x: -2, y: 0, rotation: Rotation::South},
        Position { x: -4, y: 0, rotation: Rotation::South}
    );
    let building = Building::Belt;

    let node = Node {
        recipe,
        building,
        pos: Position { x: 0, y: 0, rotation: Rotation::North},
        amount_per_sec,
    };
    grid.add(node.building.clone(), node.pos);

    generate(&mut grid, node).unwrap().to_blueprint()
}

fn generate(grid: &mut Grid, node: Node) -> Option<Grid> {
    if node.recipe.building() == Building::Miner && node.building == Building::Inserter {
        // Route to input of raw resource
        return Some(grid.clone());
        return route_raw_resource(grid.clone(), node);
    }


    // println!("inbetween blueprint: {}", grid.to_blueprint());
    let mut moves = moves(node, grid);
    if moves.len() == 0 {
        // println!("No moves available!\n");
        return None;
    }

    // Add new nodes to queue, try, and remove if failure.
    for mut possibility in moves {
        // Try adding both heads.

        if possibility.heads.len() > 1 {
            let mut new_grid = grid.clone();
            // Try placing all heads
            for head in &possibility.heads {
                if new_grid.is_free(head.building.clone(), head.pos) {
                    new_grid.add(head.building.clone(), head.pos);
                }
            }


            let mut failed_one = false;
            for head in &possibility.heads {
                match generate(&mut new_grid, head.clone()) {
                    None => { failed_one = true; break; }
                    Some(grid) => { new_grid = grid }
                }
            }
            match failed_one {
                true => continue,
                false => return Some(new_grid)
            }
        } else {
            grid.add(possibility.heads[0].building.clone(), possibility.heads[0].pos);
            let result = generate(grid, possibility.heads[0].clone());
            grid.remove(possibility.heads[0].pos);
            match result {
                None => continue,
                Some(grid) => return Some(grid),
            }
        }
    }
    None
}

fn moves(node: Node, grid: &Grid) -> Vec<Possibility> {
    match node.building {
        Building::Assembler(_) => { println!("Assembler moves"); assembler_moves(node, grid) }
        Building::Belt => { println!("belt moves"); belt_moves(node, grid) }
        Building::Inserter => { println!("inserter moves"); inserter_moves(node, grid) }
        Building::Furnace => { println!("furnace moves"); assembler_moves(node, grid) }
        _ => { unimplemented!("These buildings should not occur yet")}
    }
}


fn assembler_moves(node: Node, grid: &Grid) -> Vec<Possibility> {
    let mut possibilities = Vec::new();

    let mut positions_to_check = Vec::new();
    for i in 1..2 {
        // Top Side and Bottom Side
        positions_to_check.push(Position::new(node.pos.x + 2, node.pos.y + i - 1, Rotation::West));
        positions_to_check.push(Position::new(node.pos.x - 2, node.pos.y + i - 1, Rotation::East));
        positions_to_check.push(Position::new(node.pos.x + i - 1, node.pos.y + 2, Rotation::North));
        positions_to_check.push(Position::new(node.pos.x + i - 1, node.pos.y - 2, Rotation::South));
    }

    for p in positions_to_check {
        if grid.is_free(Building::Inserter, p) {
            possibilities.push(Possibility {
                heads: vec![Node {
                    recipe: node.recipe,
                    building: Building::Inserter,
                    pos: p,
                    amount_per_sec: node.amount_per_sec,
                }],
            });
        }
    }

    possibilities
}

fn inserter_moves(node: Node, grid: &Grid) -> Vec<Possibility> {
    let new_pos = node.pos.backward().backward();
    // we can place either a belt or an assembler.

    let mut possibilities = Vec::new();
    // Assembler
    let possible_centers = vec![new_pos]; //, new_pos.rotate_left().backward(), new_pos.rotate_right().backward()];
    println!("New pos: {:?}", new_pos);
    println!("Node pos: {:?}", node.pos);
    println!("Possible centers: {:?}", possible_centers);
    for center in possible_centers {        
        if grid.is_free(node.recipe.building(), center) {
            let mut heads = Vec::new();
            for r in node.recipe.dependencies() {
                heads.push(Node {
                    recipe: r.0,
                    building: node.recipe.building(),
                    pos: center,
                    amount_per_sec: node.amount_per_sec * r.1
                })
            }
            
            possibilities.push(Possibility { heads: heads })
        }
    }

    // belt:
    let p = node.pos.backward();
    if grid.is_free(Building::Belt, p) {
        possibilities.push(Possibility {
            heads: vec![Node {
            recipe: node.recipe,
            building: Building::Belt,
            pos: p,
            amount_per_sec: node.amount_per_sec,
        }]});
    }

    
    possibilities
}

fn belt_moves(node: Node, grid: &Grid) -> Vec<Possibility> {

    let mut possibilities = Vec::new();

    

    // We can place inserters putting something on the belts. This is a split where we have 2 heads, with reduced amount per sec, for now only consider belt backwards and inserters from the sides for convenience.
    
    let single_assembler_amount_per_sec = 1.0 / node.recipe.crafting_time();
    let leftover_amount_per_sec = node.amount_per_sec - single_assembler_amount_per_sec;

    if leftover_amount_per_sec > 0.0 {
        println!("Belt split is needed");
        let belt_position = node.pos.backward();
        if grid.is_free(Building::Belt, belt_position) {
            let inserter_position = node.pos.rotate_left().backward();
            println!("node position: {:?}, inserter_position: {:?}", node.pos, inserter_position);
            if grid.is_free(Building::Inserter, inserter_position) {
                possibilities.push(Possibility { heads: vec![
                Node {
                    recipe: node.recipe,
                    building: Building::Inserter,
                    pos: inserter_position,
                    amount_per_sec: single_assembler_amount_per_sec,
                },
                Node {
                    recipe: node.recipe,
                    building: Building::Belt,
                    pos: belt_position,
                    amount_per_sec: leftover_amount_per_sec,
                },
                ]})
            }

            let inserter_position = node.pos.rotate_right().backward();
            if grid.is_free(Building::Inserter, inserter_position) {
                possibilities.push(Possibility { heads: vec![
                Node {
                    recipe: node.recipe,
                    building: Building::Inserter,
                    pos: inserter_position,
                    amount_per_sec: single_assembler_amount_per_sec,
                },
                Node {
                    recipe: node.recipe,
                    building: Building::Belt,
                    pos: belt_position,
                    amount_per_sec: leftover_amount_per_sec,
                },
                ]})
            }
        }
    } else {
        // Just need single inserter unit without belt split.
        println!("No belt split needed");


        let inserter_position = node.pos.backward();
        if grid.is_free(Building::Inserter, inserter_position) {
            let n = Node {
                recipe: node.recipe,
                building: Building::Inserter,
                pos: inserter_position,
                amount_per_sec: node.amount_per_sec,
            };
            possibilities.push(Possibility { heads: vec![n] });
        }

        let inserter_position = node.pos.rotate_right().backward();
        if grid.is_free(Building::Inserter, inserter_position) {
            let n = Node {
                recipe: node.recipe,
                building: Building::Inserter,
                pos: inserter_position,
                amount_per_sec: node.amount_per_sec,
            };
            possibilities.push(Possibility { heads: vec![n] });
        }

        let inserter_position = node.pos.rotate_left().backward();
        if grid.is_free(Building::Inserter, inserter_position) {
            let n = Node {
                recipe: node.recipe,
                building: Building::Inserter,
                pos: inserter_position,
                amount_per_sec: node.amount_per_sec,
            };
            possibilities.push(Possibility { heads: vec![n] });
        }
    }

    // We can place a belt onto it at 3 positions:
    let position = node.pos.backward();
    if grid.is_free(Building::Belt, position) {
        possibilities.push(Possibility { heads: vec![Node {
            recipe: node.recipe,
            building: Building::Belt,
            pos: position,
            amount_per_sec: node.amount_per_sec,
        }]})
    }

    let position = node.pos.rotate_right().backward();
    if grid.is_free(Building::Belt, position) {
        possibilities.push(Possibility { heads: vec![Node {
            recipe: node.recipe,
            building: Building::Belt,
            pos: position,
            amount_per_sec: node.amount_per_sec,
        }]})
    }

    let position = node.pos.rotate_left().backward();
    if grid.is_free(Building::Belt, position) {
        possibilities.push(Possibility { heads: vec![Node {
            recipe: node.recipe,
            building: Building::Belt,
            pos: position,
            amount_per_sec: node.amount_per_sec,
        }]})
    }
    possibilities
}


fn route_raw_resource(mut grid: Grid, node: Node) -> Option<Grid> {
    let path = match node.recipe {
        Recipe::IronOre => grid.iron_ore.clone(),
        Recipe::CopperOre => grid.copper_ore.clone(),

        _ => panic!("Asking for non raw resource")
    };

    if path.len() == 1 {
        // Just route directly to the end.
        return match Astar::new(path[0].forward(), node.pos.backward(), grid.taken.clone()).astar_belt() {
            Some(path) => {
                grid.add(Building::Belt, path[0].forward());
                for p in &path {
                    grid.add(Building::Belt, *p);
                }
                // Add path to the grid resources as well.
                match node.recipe {
                    Recipe::IronOre => grid.iron_ore.append(&mut path.clone()),
                    Recipe::CopperOre => grid.copper_ore.append(&mut path.clone()),

                    _ => {}
                }
                Some(grid)
            },
            None => None
        }
    }

    for pos in path {
        // try inserter
        if !grid.is_free(Building::Inserter, pos.rotate_left().forward()) { continue; }
        if !grid.is_free(Building::Belt, pos.rotate_left().forward().forward()) { continue; }
        grid.add(Building::Inserter, pos.rotate_left().forward());
        let start = pos.rotate_left().forward().forward();
        let end = node.pos.backward();

        match Astar::new(start, end, grid.taken.clone()).astar_belt() {
            Some(path) => {
                for p in &path {
                    grid.add(Building::Belt, *p);
                }

                match node.recipe {
                    Recipe::IronOre => grid.iron_ore.append(&mut path.clone()),
                    Recipe::CopperOre => grid.copper_ore.append(&mut path.clone()),

                    _ => {}
                }

                println!("Finished first path: {}", grid.to_blueprint());
                panic!("Stopping");
                return Some(grid);
            },
            None => continue,
        }
        
    }

    None
}