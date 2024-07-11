use bevy::prelude::*;
use bevy_inspector_egui::{prelude::*, quick::WorldInspectorPlugin};
use bevy_mod_picking::prelude::*;

const GRID_SIZE: i32 = 25;

const TILE_SIZE: f32 = 64.0;

#[derive(Component)]
struct Player;


#[derive(Reflect, Component, Default, InspectorOptions, Clone, Copy, Debug)]
#[reflect(Component, InspectorOptions)]
struct Spell {
    distance: usize,
    traveled: usize,
}


/// Used to help identify our main camera
#[derive(Component)]
struct MainCamera;

#[derive(Reflect, InspectorOptions, Clone, Copy)]
#[reflect(InspectorOptions)]
enum TileState {
    None,
    Selected,
}

#[derive(Reflect, Component, Default, InspectorOptions, Clone, Copy, Debug)]
#[reflect(Component, InspectorOptions)]
struct GridLocation {
    x: i32,
    y: i32,
}

impl GridLocation {
    /**
        Helper: Return a Vec2  from the tile's x, y cords
    */
    fn to_world(&self) -> Vec2 {
        Vec2::new(self.x as f32 * TILE_SIZE, self.y as f32 * TILE_SIZE)
    }
}

#[derive(Reflect, Component, InspectorOptions, Clone, Copy)]
#[reflect(Component, InspectorOptions)]
struct Tile {
    location: GridLocation,
    state: TileState,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .register_type::<Tile>()
        .register_type::<GridLocation>()
        .register_type::<Spell>()
        .add_systems(Startup, setup)
        .add_systems(Update, (update_grid_position).chain())
        .run();
}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    // set up grid
    for x in -GRID_SIZE..GRID_SIZE {
        for y in -GRID_SIZE..GRID_SIZE {
            let tile = Tile {
                location: GridLocation { x: x, y: y },
                state: TileState::None,
            };

            let tile_bundle = (
                Name::new("tile"),
                tile,
                SpriteBundle {
                    texture: asset_server.load("tile.png"),
                    transform: Transform {
                        translation: tile.location.to_world().extend(0.0),
                        ..default()
                    },
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(TILE_SIZE)),
                        ..default()
                    },
                    ..default()
                },
            );
            commands.spawn(tile_bundle);
        }
    }

    // spawn player

    let player_bundle = (
        Name::new("player"),
        Player,
        SpriteBundle {
            texture: asset_server.load("player.png"),
            sprite: Sprite {
                anchor: bevy::sprite::Anchor::BottomCenter,
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..default()
            },
            ..default()
        },
        GridLocation { x: 1, y: 1 },
    );

    commands.spawn(player_bundle);
}


fn update_grid_position(
    mut grid_query: Query<(&GridLocation, &mut Transform)>,
) {
    for (grid_location, mut transform) in grid_query.iter_mut() {
        transform.translation = grid_location.to_world().extend(0.1).clone();
    }
}
