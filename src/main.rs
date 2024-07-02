use bevy::prelude::*;
mod components;

fn setup(
    mut commands: Commands
) {
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
        ))
        .add_systems(Startup, (setup).chain())
        .run();
}
