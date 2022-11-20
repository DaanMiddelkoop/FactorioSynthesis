use crate::{recipes::{Recipe, State}, grid::Grid, rotation::Rotation, building::Building, entity::Entity, position::Position};




pub fn synth(recipe: Recipe, amount_per_sec: f64) -> Grid {
    // synth this new step

    let mut row = synth_row(recipe, (amount_per_sec * recipe.crafting_time()).ceil() as isize);


    for (index, (r, a)) in recipe.dependencies().drain().enumerate() {
        println!("Synthesizing {:?} for {:?}", r, recipe);
        let child_grid = synth(r, a * amount_per_sec);
        row.add(child_grid, index);
    }

    row
}

pub fn synth_row(recipe: Recipe, row_length: isize) -> Grid {

    


    let mut grid = Grid::new(recipe);
    let (building_width, building_height) = recipe.building().size();

    if recipe.building() == Building::Miner {
        grid.place_belts(0, 0, Rotation::West, row_length * building_width);
        grid.set_output(Position { x: 0, y: 0, rotation: Rotation::West});
        return grid;
    }

    if recipe.output_state().is_liquid() {
        grid.set_output(Position {x: 0, y: 0, rotation: Rotation::North});
        return grid;
    }

    let input_count = recipe.dependencies().len() as isize;
    let fluid_input_count = recipe.dependencies().iter().filter(|x| {x.0.output_state().is_liquid()}).count() as isize;
    let building_entity = Entity::from(recipe.building());

    assert!(fluid_input_count < 2);

    // Assemble output row:
    match recipe.output_state() {
        State::Solid => {
            grid.place_belts(0, 0, Rotation::West, row_length * building_width);
            grid.set_output(Position { x: 0, y: 0, rotation: Rotation::West });
        }

        State::Liquid => {
            todo!()
        }
    }

    // Place building.
    if input_count > 5 || (fluid_input_count > 0 && input_count > 3) { // We need to move the building lower as there has to be another input above the building.

        if fluid_input_count > 0 {
            grid.place_inserters(1, 0, Rotation::North, row_length, building_width);
            grid.place_entity(building_entity, 3, 1, Rotation::North, row_length, building_width);
            grid.place_pipe_tunnel(1, 1, Rotation::North, row_length, building_width);
            grid.place_pipe_tunnel(-1, 1, Rotation::South, row_length, building_width);
            grid.place_pipe(-2, 0, Rotation::North, row_length * 3, 1);
        }

    } else {
        // grid.place_poles(1, 0, Rotation::North, row_length, building_width);
        grid.place_inserters(1, 1, Rotation::North, row_length, building_width);
        grid.place_entity(building_entity, 3, 1, Rotation::North, row_length, building_width);
    }

    // Assemble input rows
    if input_count <= 2 && fluid_input_count == 0 {
        grid.place_inserters(2 + building_height, 1, Rotation::North, row_length, building_width);
        // grid.place_poles(2 + building_height, 0, Rotation::North, row_length, building_width);
        grid.place_belts(3 + building_height, -1, Rotation::East, row_length * building_width + 1);


        // Both items should go to this input, but one to the top and one to the bottom half, gonna be exciting.
        grid.add_input(Position { x: -1, y: 4 + building_height, rotation: Rotation::South});
        grid.add_input(Position { x: -1, y: 2 + building_height, rotation: Rotation::North});

    } else if (input_count <= 4 && fluid_input_count == 0) || (input_count <= 5 && fluid_input_count == 1) {
        
        grid.place_inserters(2 + building_height, 1, Rotation::North, row_length, building_width);
        // grid.place_poles(2 + building_height, 0, Rotation::North, row_length, building_width);
        grid.place_long_inserters(2 + building_height, 2, Rotation::North, row_length, building_width);
        grid.place_belts(3 + building_height, -1, Rotation::East, row_length * building_width + 1);
        grid.place_belts(4 + building_height, -2, Rotation::East, row_length * building_width + 2);

        // Both items should go to this input, but one to the top and one to the bottom half, gonna be exciting.
        grid.place_belt(-1, 2 + building_height, Rotation::North);
        grid.place_belt(-1, 1 + building_height, Rotation::North);
        grid.place_belt(-2, 1 + building_height, Rotation::East);

        grid.add_input(Position { x: -2, y: 5 + building_height, rotation: Rotation::South});
        grid.add_input(Position { x: -2, y: 3 + building_height, rotation: Rotation::North});
        grid.add_input(Position { x: -2, y: 2 + building_height, rotation: Rotation::South});
        grid.add_input(Position { x: -2, y: 0 + building_height, rotation: Rotation::North});
    } else {
        todo!()
    }

    grid
}