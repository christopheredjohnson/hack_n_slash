use bevy::prelude::*;

#[derive(Component)]
struct Player {
    speed: f32,
}

#[derive(Component)]
struct Weapon {
    shape: Shape,
}

#[derive(Component)]
enum Shape {
    Stick,
}

#[derive(Component)]
struct SwipeAttack {
    duration: Timer,
    direction: Vec3,
}

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());

    // you can also use `with_children`:
    commands
        .spawn((
            Player { speed: 2.00 },
            SpriteBundle {
                texture: asset_server.load("icon.png"),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(50.00, 50.00)),
                    ..default()
                },
                ..default()
            },
        ));
        // .with_children(|parent| {
        //     parent.spawn((
        //         Weapon {
        //             shape: Shape::Stick,
        //         },
        //         SpriteBundle {
        //             transform: Transform::from_xyz(10.0, 0.0, 1.0),
        //             sprite: Sprite {
        //                 color: Color::BLACK,
        //                 custom_size: Some(Vec2::new(5.00, 100.00)),
        //                 ..default()
        //             },
        //             ..default()
        //         },
        //     ));
        // });
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut transform_query: Query<(&mut Transform, &Player), With<Player>>,
) {
    let (mut player_transform, player) = transform_query.single_mut();

    let mut direction: Vec3 = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if direction.length() > 0.0 {
        direction = direction.normalize();
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        player_transform.rotate_z(0.1);
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        player_transform.rotate_z(-0.1);
    }

    player_transform.translation += direction * player.speed;
}

fn player_attack(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut player_query: Query<(&Transform, &Children), With<Player>>,
) {


    let (mut transform, children) = player_query.single_mut();
    
    // if keyboard_input.just_pressed(KeyCode::Space) {
    //     info!("Attack!")
    // }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement, player_attack).chain())
        .run();
}
