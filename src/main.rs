use bevy::prelude::*;
use bevy_inspector_egui::{egui::Shape, quick::WorldInspectorPlugin, InspectorOptions};

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Resource)]
pub struct LastMoveTime(f32);

#[derive(Resource, InspectorOptions)]
pub struct Grid {
    size: Vec2,
    cell_size: Vec2,
}

impl Grid {
    fn world_to_grid(&self, world_pos: Vec2) -> IVec2 {
        IVec2::new(
            (world_pos.x / self.cell_size.x).floor() as i32,
            (world_pos.y / self.cell_size.y).floor() as i32,
        )
    }

    fn grid_to_world(&self, grid_pos: IVec2) -> Vec2 {
        Vec2::new(
            grid_pos.x as f32 * self.cell_size.x + self.cell_size.x / 2.0,
            grid_pos.y as f32 * self.cell_size.y + self.cell_size.y / 2.0,
        )
    }
}

#[derive(Component, InspectorOptions)]
pub struct Player;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.insert_resource(Grid {
        size: Vec2::new(20.0, 20.0),
        cell_size: Vec2::new(32.0, 32.0),
    });
    commands.spawn(Camera2dBundle::default());

    let texture = asset_server.load("Dungeon_Character.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 7, 2, None, None);

    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 1, last: 6 };

    commands.spawn((
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            sprite: Sprite {
                custom_size: Some(Vec2::splat(32.0)),
                ..default()
            },
            // transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
        Player,
    ));
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
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

        let new_grid_pos = if keyboard_input.pressed(KeyCode::KeyW) {
            IVec2::new(grid_pos.x, grid_pos.y + 1)
        } else if keyboard_input.pressed(KeyCode::KeyS) {
            IVec2::new(grid_pos.x, grid_pos.y - 1)
        } else if keyboard_input.pressed(KeyCode::KeyA) {
            IVec2::new(grid_pos.x - 1, grid_pos.y)
        } else if keyboard_input.pressed(KeyCode::KeyD) {
            IVec2::new(grid_pos.x + 1, grid_pos.y)
        } else {
            grid_pos
        };

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

fn main() {
    App::new()
        // .insert_resource(ClearColor(Color::GREEN))
        .insert_resource(LastMoveTime(0.0))
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .add_systems(Update, (player_input, snap_to_grid, animate_sprite).chain())
        .run();
}
