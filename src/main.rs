use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_pancam::{PanCam, PanCamPlugin};
use components::{Grid, LastMoveTime, MainCamera, Player};

mod components;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut grid = Grid {
        size: Vec2::new(5.0, 5.0),
        cell_size: Vec2::new(16.0, 16.0),
    };

    commands.insert_resource(grid);

   

    let player_texture: Handle<Image> = asset_server.load("Character.png");
    let player_layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 7, 2, None, None);

    let player_texture_atlas_layout = texture_atlas_layouts.add(player_layout);
    let starting_point = grid.grid_to_world(IVec2::new(1, 1));

    commands.spawn((Camera2dBundle::default(), PanCam::default(), MainCamera));
    
    commands.spawn((
        SpriteSheetBundle {
           sprite: Sprite {
            custom_size: Some(Vec2::splat(16.0)),
            ..default()
           },
            ..default()
        },
        Player,
    ));
}

fn player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    grid: Res<Grid>,
    mut query: Query<(&mut Transform, &Player)>,
    mut last_move_time: ResMut<LastMoveTime>,
    time: Res<Time>,
) {
    let now = time.elapsed_seconds();
    
    if now - last_move_time.0 < 0.2 {
        return;
    }

    for (mut transform, _player) in query.iter_mut() {
        let grid_pos = grid.world_to_grid(transform.translation.truncate());
        let mut new_grid_pos = grid_pos;

        if keyboard_input.pressed(KeyCode::KeyW) {
            new_grid_pos.y += 1;
        } else if keyboard_input.pressed(KeyCode::KeyS) {
            new_grid_pos.y -= 1;
        } else if keyboard_input.pressed(KeyCode::KeyA) {
            new_grid_pos.x -= 1;
        } else if keyboard_input.pressed(KeyCode::KeyD) {
            new_grid_pos.x += 1;
        }

        info!("{:?}", new_grid_pos);

        if new_grid_pos != grid_pos {
            transform.translation = grid.grid_to_world(new_grid_pos).extend(0.0);
            last_move_time.0 = now;
        }
    }
}

fn snap_to_grid(grid: Res<Grid>, mut query: Query<(&mut Transform, &Player)>) {
    for (mut transform, _player) in query.iter_mut() {
        let grid_pos = grid.world_to_grid(transform.translation.truncate());
        transform.translation = grid.grid_to_world(grid_pos).extend(0.0);
    }
}

fn camera_follow (
    grid: Res<Grid>,
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>
) {
    let player_transform: &Transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.y = player_transform.translation.y;
    camera_transform.translation.x = player_transform.translation.x;
}



fn main() {
    App::new()
        .insert_resource(ClearColor(Color::GREEN))
        .insert_resource(LastMoveTime(0.0))
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
            PanCamPlugin::default(),
        ))
        .add_systems(Startup, (setup).chain())
        .add_systems(Update, (player_input, snap_to_grid, camera_follow).chain())
        .run();
}
