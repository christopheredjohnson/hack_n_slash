use bevy::prelude::*;

const GRID_SIZE: u8 = 20;


#[derive(Component)]
struct Grid;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}
