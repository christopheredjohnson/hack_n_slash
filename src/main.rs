use ::bevy::prelude::*;
use player::PlayerPlugin;


mod player;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Hack N Slash!".to_string(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
