use bevy::{prelude::*, window::WindowResolution};

const TILE_SIZE: f32 = 40.0;

#[derive(Component, Debug)]
struct LevelLocation {
    x: usize,
    y: usize,
}

#[derive(Resource, Clone, Copy)]
struct Level {
    width: usize,
    height: usize,
}

fn main() {
    let level = Level {
        width: 20,
        height: 20,
    };

    App::new()
        .insert_resource(level)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Hack N Slash!".to_string(),
                resolution: WindowResolution::new(
                    level.width as f32 * TILE_SIZE,
                    level.height as f32 * TILE_SIZE,
                ),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, intital_setup)
        .add_systems(Update, (button_system).chain())
        .run();
}

fn intital_setup(mut commands: Commands, level: Res<Level>) {
    commands.spawn(Camera2dBundle::default());

    //Button style
    let button_style = Style {
        display: Display::Grid,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(1.0)),
        ..default()
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                //Create a grid layout,
                display: Display::Grid,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                grid_template_columns: vec![GridTrack::auto()],
                //Top Row will take up all the space after the bottom row is complete.
                grid_template_rows: vec![GridTrack::flex(1.0)],
                ..default()
            },
            background_color: BackgroundColor(Color::WHITE),
            ..default()
        })
        .with_children(|builder| {
            //Game Area
            builder
                .spawn(NodeBundle {
                    style: Style {
                        //Create a grid layout,
                        display: Display::Grid,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        grid_template_columns: vec![GridTrack::auto(); level.width],
                        grid_template_rows: vec![GridTrack::auto(); level.height],
                        ..default()
                    },
                    background_color: BackgroundColor(Color::WHITE),
                    ..default()
                })
                .with_children(|game_area_builder| {
                    //         //Every other will be black or white!
                    for c in 0..level.width {
                        for r in 0..level.height {
                            game_area_builder.spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: BackgroundColor(Color::GREEN),
                                    border_color: BorderColor(Color::DARK_GREEN),
                                    ..default()
                                },
                                LevelLocation { x: c, y: r },
                            ));
                        }
                    }
                });
        });
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &LevelLocation),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, grid_loc) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                info!("{:?}", grid_loc)
            },
            Interaction::Hovered | Interaction::None => {}
        }
    }
}
