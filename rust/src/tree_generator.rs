use crate::{position::Position, recipes::Recipe, building::Building, grid::Grid, entity::Entity, rotation::Rotation};
use rand::seq::SliceRandom; // 0.7.2


#[derive(Debug)]
pub struct Possibility {
    heads: Vec<Node>,
}

#[derive(Clone, Debug)]
pub struct Node {
    recipe: Recipe,
    entity: Entity,
    pos: Position,
    amount_per_sec: f64,
}

pub fn generate_recipe(recipe: Recipe, amount_per_sec: f64) -> Grid {
    let mut grid = Grid::new(recipe);

    let start_node = Node {
        recipe: recipe,
        entity: Entity::from(Building::Belt),
        pos: Position::new(0, 0, Rotation::North),
        amount_per_sec: amount_per_sec,
    };

    let mut entity = start_node.entity.clone();
    entity.set_position(start_node.pos);
    grid.add_entity(entity);


    let mut queue = vec![start_node];
    generate(&mut queue, &mut grid).unwrap()
}



pub fn generate(queue: &mut Vec<Node>, grid: &Grid) -> Option<Grid> {
    println!("Queue: {:?}", queue);

    // Check if any node has no moves left as that would be bad
    for n in queue.iter() {
        if moves(n.clone(), grid).len() == 0 && n.recipe.building() != Building::Miner {
            return None;
        }
    }

    if queue.len() == 0 {
        return Some(grid.clone());
    }


    let node = queue.remove(0);
    println!("Trying new node: {:?} {:?} {:?} queue len: {}", node.entity.building, node.recipe, node.pos, queue.len());
    // if entity.position.y > 2 || entity.position.y < -2 {
    //     panic!()
    // }

    match node.recipe.building() {
        Building::Miner => { return generate(queue, grid) }

        _ => ()
    }

    println!("partial blueprint: {}", grid.to_blueprint());

    if node.recipe.dependencies().len() == 0 {
        return None;
    }


    let mut possible_moves = moves(node, grid);
    // possible_moves.shuffle(&mut rand::thread_rng());
    println!("\nPossible moves\n");
    for m in &possible_moves {
        print!("\tHead(");
        for h in &m.heads {
            print!("{:?}: {:?} - ", h.pos, h.entity.building);
        }
        println!(")")
    }
    if possible_moves.len() == 0 {
        return None;
    }


    // Add new nodes to queue, try, and remove if failure.
    for mut possibility in possible_moves {
        let original_entities = grid.entity_amount();

        let mut new_queue = queue.to_vec();
        let mut new_grid = grid.clone();
        // place all heads of the new possibility in the grid:
        let mut entities = Vec::new();
        for head in &possibility.heads {
            let mut entity = head.entity.clone();
            entity.set_position(head.pos);
            entities.push(entity.clone());
            new_grid.add_entity(entity);
        }

        new_queue.append(&mut possibility.heads);
        if let Some(grid) = generate(&mut new_queue, &new_grid) {
            return Some(grid);
        }

        assert!(grid.entity_amount() == original_entities);
    }
    return None;
}

pub fn moves(node: Node, grid: &Grid) -> Vec<Possibility> {
    match node.entity.building {
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
        if grid.is_free(p) {
            possibilities.push(Possibility {
                heads: vec![Node {
                    recipe: node.recipe,
                    entity: Entity::from(Building::Inserter),
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
    println!("Node pos: {:?}", node.pos);
    println!("Possible centers: {:?}", possible_centers);
    for center in possible_centers {
        let mut collided = false;
        for x in 0..3 {
            for y in 0..3 {
                collided |= !grid.is_free(Position::north(center.x + x - 1, center.y + y - 1));
            }
        }
        
        if !collided {
            let mut heads = Vec::new();
            for r in node.recipe.dependencies() {
                heads.push(Node {
                    recipe: r.0,
                    entity: Entity::from(node.recipe.building()),
                    pos: center,
                    amount_per_sec: node.amount_per_sec * r.1
                })
            }
            
            possibilities.push(Possibility { heads: heads })
        }
    }

    // belt:
    let p = node.pos.backward();
    if grid.is_free(p) {
        possibilities.push(Possibility {
            heads: vec![Node {
            recipe: node.recipe,
            entity: Entity::from(Building::Belt),
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
        if grid.is_free(belt_position) {
            let inserter_position = node.pos.rotate_left().backward();
            println!("node position: {:?}, inserter_position: {:?}", node.pos, inserter_position);
            if grid.is_free(inserter_position) {
                possibilities.push(Possibility { heads: vec![
                Node {
                    recipe: node.recipe,
                    entity: Entity::from(Building::Inserter),
                    pos: inserter_position,
                    amount_per_sec: single_assembler_amount_per_sec,
                },
                Node {
                    recipe: node.recipe,
                    entity: Entity::from(Building::Belt),
                    pos: belt_position,
                    amount_per_sec: leftover_amount_per_sec,
                },
                ]})
            }

            let inserter_position = node.pos.rotate_right().backward();
            if grid.is_free(inserter_position) {
                possibilities.push(Possibility { heads: vec![
                Node {
                    recipe: node.recipe,
                    entity: Entity::from(Building::Inserter),
                    pos: inserter_position,
                    amount_per_sec: single_assembler_amount_per_sec,
                },
                Node {
                    recipe: node.recipe,
                    entity: Entity::from(Building::Belt),
                    pos: belt_position,
                    amount_per_sec: leftover_amount_per_sec,
                },
                ]})
            }
        }
    } else {
        // Just need single inserter unit without belt split.
        println!("No belt split needed");


        let inserter_position = node.pos.rotate_right().forward().rotate_left().rotate_left();
        if grid.is_free(inserter_position) {
            let n = Node {
                recipe: node.recipe,
                entity: Entity::from(Building::Inserter),
                pos: inserter_position,
                amount_per_sec: node.amount_per_sec,
            };
            possibilities.push(Possibility { heads: vec![n] });
        }

        let inserter_position = node.pos.rotate_left().forward().rotate_left().rotate_left();
        if grid.is_free(inserter_position) {
            let n = Node {
                recipe: node.recipe,
                entity: Entity::from(Building::Inserter),
                pos: inserter_position,
                amount_per_sec: node.amount_per_sec,
            };
            possibilities.push(Possibility { heads: vec![n] });
        }

        let inserter_position = node.pos.backward().rotate_left().rotate_left();
        if grid.is_free(inserter_position) {
            let n = Node {
                recipe: node.recipe,
                entity: Entity::from(Building::Inserter),
                pos: inserter_position,
                amount_per_sec: node.amount_per_sec,
            };
            possibilities.push(Possibility { heads: vec![n] });
        }
    }

    // We can place a belt onto it at 3 positions:
    let position = node.pos.backward();
    if grid.is_free(position) {
        possibilities.push(Possibility { heads: vec![Node {
            recipe: node.recipe,
            entity: Entity::from(Building::Belt),
            pos: position,
            amount_per_sec: node.amount_per_sec,
        }]})
    }

    let position = node.pos.rotate_right().backward();
    if grid.is_free(position) {
        possibilities.push(Possibility { heads: vec![Node {
            recipe: node.recipe,
            entity: Entity::from(Building::Belt),
            pos: position,
            amount_per_sec: node.amount_per_sec,
        }]})
    }

    let position = node.pos.rotate_left().backward();
    if grid.is_free(position) {
        possibilities.push(Possibility { heads: vec![Node {
            recipe: node.recipe,
            entity: Entity::from(Building::Belt),
            pos: position,
            amount_per_sec: node.amount_per_sec,
        }]})
    }
    possibilities
}

