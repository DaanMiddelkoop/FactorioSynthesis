use grid::Grid;
use recipes::Recipe;

mod building;
mod recipes;
mod synthesize;
mod position;
mod rotation;
mod grid;
mod entity;
mod bounds;
mod astar;

fn main() {

    // let mut grid = Grid::new();
    // grid.place_belts(0, rotation::Rotation::North, 1);
    // println!("Grid blueprint string: {}", grid.to_blueprint());
    let grid = synthesize::synth(Recipe::BigElectricMotor, 0.5);

    println!("blueprint: {}", grid.to_blueprint());


}
