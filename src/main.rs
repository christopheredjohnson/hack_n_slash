use bevy::{prelude::*, transform, window::WindowResolution};

const TILE_SIZE: f32 = 60.0;

#[derive(Component, Debug)]
struct Player;

#[derive(Component, Debug)]
struct CurrentLevelLocation {
    x: usize,
    y: usize,
}

#[derive(Component, Debug)]
struct TileState {
    selected: bool
}

#[derive(Resource, Clone)]
struct Level {
    width: usize,
    height: usize,
    tiles: Vec<Vec<usize>>,
}

fn main() {
    let mut level = Level {
        width: 0,
        height: 0,
        tiles: vec![
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ],
    };

    // calculate the height, and width from tiles
    level.height = level.tiles.len();

    level.width = if level.height > 0 {
        level.tiles[0].len()
    } else {
        0
    };

    App::new()
        .insert_resource(level.clone())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Hack N Slash!".to_string(),
                resolution: WindowResolution::new(
                    (level.width * TILE_SIZE as usize) as f32,
                    (level.height * TILE_SIZE as usize) as f32,
                ),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, intital_setup)
        .add_systems(Update, (button_system, highlight_tiles).chain())
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
                    game_area_builder.spawn((
                        SpriteBundle { ..default() },
                        Player,
                        CurrentLevelLocation { x: 0, y: 0 },
                    ));
                    //Every other will be black or white!
                    for c in 0..level.width {
                        for r in 0..level.height {
                            game_area_builder.spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    border_color: BorderColor(Color::BLACK),
                                    ..default()
                                },
                                CurrentLevelLocation { x: c, y: r },
                                TileState {
                                   selected: false
                                }
                            ));
                        }
                    }
                });
        });
}




fn button_system(
    mut interaction_query:
        Query<
        (Entity, &Interaction),
        (Changed<Interaction>, With<Button>),
    >,
    mut tile_query:  Query<(&mut TileState)>
) {

    
    for (entity_id, interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {

                // clear current selected tiles
                for mut tile_state in &mut tile_query {
                    tile_state.selected = false;
                }

                // select tile
                if let Ok(mut tile_state) = tile_query.get_mut(entity_id) {
                    tile_state.selected = true;
                }

            }
            Interaction::Hovered | Interaction::None => {}
        }
    }
}

fn highlight_tiles (
    mut tile_query:  Query<(&TileState, &mut BorderColor)>
) {
    for (tile , mut border_color) in &mut tile_query {

        let color = if tile.selected {
            Color::RED
        } else {
            Color::BLACK
        };
        
        *border_color = BorderColor(color);
    }
}

