use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    speed: f32
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    commands.spawn(camera);
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(50.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        Player {
            speed: 2.0
        },
    ));
}

fn player_movement(
  time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Player, &mut Transform), With<Player>>,
) {
    let (player, mut player_transform) = player_query.single_mut();

    let player_speed = player.speed * 100.0;


    if keyboard.pressed(KeyCode::KeyW) {
            player_transform.translation.y +=  player_speed * time.delta_seconds();
    }

    if keyboard.pressed(KeyCode::KeyS) {
            player_transform.translation.y -=  player_speed * time.delta_seconds();
    }

    if keyboard.pressed(KeyCode::KeyA) {
            player_transform.translation.x -=  player_speed * time.delta_seconds();
    }

    if keyboard.pressed(KeyCode::KeyD) {
            player_transform.translation.x +=  player_speed * time.delta_seconds();
    }

    info!("{:?}", player_transform.translation);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.3, 0.5, 0.3)))
        .add_systems(Startup, (spawn_camera, spawn_player))
        .add_systems(Update, player_movement)
        .run();
}
