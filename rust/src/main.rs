#![allow(dead_code)]

use grid::Grid;
use position::Position;
use recipes::Recipe;
use spring_system::SpringSystem;

mod building;
mod recipes;
mod synthesize;
mod position;
mod rotation;
mod grid;
mod entity;
mod bounds;
mod astar;
mod tree_generator;
mod spring_system;

fn main() {

    // let mut grid = Grid::new();
    // grid.place_belts(0, rotation::Rotation::North, 1);
    // println!("Grid blueprint string: {}", grid.to_blueprint());
    // let grid = synthesize::synth(Recipe::BigElectricMotor, 0.5);

    // let position = Position::new(0, 0, rotation::Rotation::North);
    // println!("Backwards: {:?}, forwards: {:?}, left: {:?}, right: {:?}", position.backward(), position.forward(), position.rotate_left(), position.rotate_right());

    let blueprint = tree_generator::generate_recipe(Recipe::FastInserter, 3.0);
    println!("blueprint: {}", blueprint);

    // let system = SpringSystem::new(Recipe::FastInserter, 30.0);
    // system.partial_blueprint();

}
