use super::GameState;
use crate::{
    chunks::{WorldConfiguration, ChunkData, BoardPosition, Cell, Tile},
    pawn::{free_movement::FirstPersonPawn, Player},
    pickup::{spawn_a_pickup, Pickup},
    ui::score::DataDisplay,
    Game,
};
use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use rand::Rng;
use std::f32::consts::PI;

/// reset the game to a valid initial state
pub fn setup(
    mut commands: Commands,
    mut game: ResMut<Game>,
    cameras: Query<Entity, With<Camera>>,
    boards: Res<Assets<WorldConfiguration>>,
) {
    game.cake_eaten = 0;
    game.score = 0;
    let Some(board_config) = boards.get(game.world_handle.clone()) else {
        warn!("expected board config asset to be loaded by now");
        return;
    };

    let board = commands.spawn(SpatialBundle::default()).id();
    let board_pieces: Vec<_> = (0..board_config.height)
        .flat_map(|y| {
            (0..board_config.width).map(move |x| (x, y, rand::thread_rng().gen_range(-0.1..0.1)))
        })
        .map(|(x, y, height)| {
            let entity = commands
                .spawn((
                    SceneBundle {
                        transform: Transform::from_xyz(x as f32, height - 0.2, y as f32),
                        scene: game.tile_handle.clone(),
                        ..default()
                    },
                    Cell { height },
                    BoardPosition { x, y },
                ))
                .id();
            commands.entity(board).add_child(entity);
            (x, y, entity.clone(), height)
        })
        .collect();

    let board_cache: ChunkData = ChunkData::new(
        board_config.width,
        board_config.height,
        board_pieces
            .into_iter()
            .map(|(_x, _y, entity, height)| Tile { entity, height })
            .collect(),
    );

    let Ok(camera) = cameras.get_single() else {
        warn!("unexpected camera count");
        return;
    };

    let player_position = BoardPosition {
        x: board_config.width / 2,
        y: board_config.height / 2,
    };

    let translation = Vec3::new(
        player_position.x as f32,
        board_cache.get_height(player_position.x, player_position.y),
        player_position.y as f32,
    );

    commands.entity(board).insert(board_cache);

    let rotation = Quat::from_rotation_y(-PI / 2.);

    commands
        .spawn(SceneBundle {
            transform: Transform {
                translation,
                rotation,
                ..default()
            },
            scene: game.player_handle.clone(),
            ..default()
        })
        .insert((
            player_position,
            Player,
            // Facing::Up,
            // InputRateLimit(Timer::new(Duration::from_millis(200), TimerMode::Once)),
            // FollowMe::new(camera),
            FirstPersonPawn::new(camera),
        ));

    commands.spawn((
        DataDisplay::Score,
        TextBundle::from_section(
            "Score:",
            TextStyle {
                font_size: 40.0,
                color: Color::rgb(0.5, 0.5, 1.0),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
    ));

    commands.spawn((
        DataDisplay::PickupPosition,
        TextBundle::from_section(
            "Cake:",
            TextStyle {
                font_size: 40.0,
                color: Color::rgb(0.5, 0.5, 1.0),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(50.0),
            left: Val::Px(5.0),
            ..default()
        }),
    ));

    commands.spawn((
        DataDisplay::PlayerPosition,
        TextBundle::from_section(
            "Cake:",
            TextStyle {
                font_size: 40.0,
                color: Color::rgb(0.5, 0.5, 1.0),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(90.0),
            left: Val::Px(5.0),
            ..default()
        }),
    ));

    {
        let x = rand::thread_rng().gen_range(0..board_config.width);
        let y = rand::thread_rng().gen_range(0..board_config.height);

        spawn_a_pickup(&mut commands, game.pickup_handle.clone(), x, y, 0.0);
    }
}

/// remove all entities from the scene except cameras and windows
pub fn teardown(
    mut commands: Commands,
    entities: Query<Entity, (Without<Camera>, Without<Window>)>,
) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}

/// end the game if score is below a threshold
pub fn end_game(game: Res<Game>, mut next_state: ResMut<NextState<GameState>>) {
    if game.score <= -500 {
        next_state.set(GameState::GameOver);
    }
}

/// update the text of the scoreboard
pub fn update_scoreboard(
    game: Res<Game>,
    mut query: Query<(&mut Text, &DataDisplay)>,
    pickups: Query<&BoardPosition, (With<Pickup>, Without<DataDisplay>, Without<Player>)>,
    players: Query<(&BoardPosition, &FirstPersonPawn), (With<Player>, Without<DataDisplay>)>,
) {
    for (mut text, data) in query.iter_mut() {
        use DataDisplay as DD;
        match data {
            DD::Score => text.sections[0].value = format!("Score: {}", game.score),
            DD::PickupPosition => {
                let Some(bp) = pickups.iter().next() else {
                    continue;
                };
                text.sections[0].value = format!("Cake: {},{}", bp.x, bp.y);
            }
            DD::PlayerPosition => {
                let Some((bp, fpp)) = players.iter().next() else {
                    continue;
                };
                text.sections[0].value =
                    format!("Player: {},{}, look {}", bp.x, bp.y, fpp.look_rotation);
            }
        }
    }
}

pub fn enter_menu(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Menu);
        let mut window = windows.get_single_mut().unwrap();
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}
