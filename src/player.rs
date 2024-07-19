use benimator::FrameRate;
use ::bevy::prelude::*;

// Create the animation component
// Note: you may make the animation an asset instead of a component
#[derive(Component, Deref, Debug)]
struct Animation(benimator::Animation);

#[derive(Default, Component, Deref, DerefMut)]
struct AnimationState(benimator::State);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (player_direction, animate).chain());
    }
}

#[derive(Component)]
struct Player;


#[derive(Component, Default)]
enum Direction {
    North,
    East,
    #[default]
    South,
    West
}

fn animate(
    time: Res<Time>,
    mut query: Query<(&mut AnimationState, &mut TextureAtlas, &Animation)>,
) {
    for (mut player, mut texture, animation) in query.iter_mut() {
        // Update the state
        player.update(animation, time.delta());

        // Update the texture atlas
        texture.index = player.frame_index();
    }
}


fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {


    // Create an animation
    let animation = Animation(benimator::Animation::from_indices(
        0..=5,
        FrameRate::from_fps(10.0),
    ));


    let texture: Handle<Image> = asset_server.load("characters/player.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(48.0, 48.0), 6, 6, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
            transform: Transform::from_scale(Vec3::splat(3.0)),
            ..default()
        },
        Player,
        Direction::default(),
        animation,
        AnimationState::default()
    ));
}

fn player_direction (
    mut commands: Commands,
    mut player_query: Query<(&Direction, &mut Animation), With<Player>>,
) {
    let (direction, mut animation) = player_query.single_mut();
    match direction {
        Direction::South => {
            animation.0 = benimator::Animation::from_indices(
                0..=5,
                FrameRate::from_fps(10.0),
            );
        }
        Direction::East => {
            animation.0 = benimator::Animation::from_indices(
                6..=11,
                FrameRate::from_fps(10.0),
            );
        }
        Direction::North => {
            animation.0 = benimator::Animation::from_indices(
                12..=17,
                FrameRate::from_fps(10.0),
            );
        }
        Direction::West => {
            // animation.0 = benimator::Animation::from_indices(
            //     18..=23,
            //     FrameRate::from_fps(10.0),
            // );
        }
    }

}