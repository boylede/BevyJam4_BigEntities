use bevy::prelude::*;

use crate::{
    board::{Board, RESET_FOCUS},
    ui::{Trigger, UiCommand, UiCommands},
    Game,
};

use super::GameState;

const TILE_PATH: &str = "embedded://big_entities_lib/assets/models/AlienCake/tile.glb#Scene0";
const PLAYER_PATH: &str = "embedded://big_entities_lib/assets/models/AlienCake/alien.glb#Scene0";
const PICKUP_PATH: &str =
    "embedded://big_entities_lib/assets/models/AlienCake/cakeBirthday.glb#Scene0";
const WORLD_PATH: &str = "worlds/main.world.ron";

/// set up the camera, light, and loading ui's, as well as initiate model asset loading
pub fn setup_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(
            -(4.0 as f32 / 2.0),
            2.0 * 4.0 as f32 / 3.0,
            4.0 as f32 / 2.0 - 0.5,
        )
        .looking_at(Vec3::from(RESET_FOCUS), Vec3::Y),
        ..default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 10.0, 4.0),
        point_light: PointLight {
            intensity: 3000.0,
            shadows_enabled: true,
            range: 30.0,
            ..default()
        },
        ..default()
    });

    let tile_handle: Handle<Scene> = asset_server.load(TILE_PATH);
    let player_handle: Handle<Scene> = asset_server.load(PLAYER_PATH);
    let pickup_handle: Handle<Scene> = asset_server.load(PICKUP_PATH);
    let world_handle: Handle<Board> = asset_server.load(WORLD_PATH);

    create_ui(
        &mut commands,
        vec![
            tile_handle.clone(),
            player_handle.clone(),
            pickup_handle.clone(),
        ],
    );

    let game = Game {
        score: 0,
        cake_eaten: 0,
        player_handle,
        pickup_handle,
        tile_handle,
        world_handle,
    };
    commands.insert_resource(game);
}

fn create_ui(commands: &mut Commands, scenes: impl IntoIterator<Item = Handle<Scene>>) {
    let mut ready_ui_commands = commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        visibility: Visibility::Hidden,
        ..default()
    });

    let ready_ui = ready_ui_commands.id();
    let button_commands = UiCommands::builder()
        .with(UiCommand::SwitchState(GameState::Playing))
        .with(UiCommand::CaptureMouse)
        .with(UiCommand::Despawn(ready_ui));

    ready_ui_commands.with_children(|parent| {
        parent
            .spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(150.0),
                    height: Val::Px(65.0),
                    border: UiRect::all(Val::Px(5.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..default()
            })
            .insert(button_commands)
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Play!",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ));
            });
    });

    let mut loading_ui_commands = commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    });
    loading_ui_commands.with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Loading...",
            TextStyle {
                font_size: 64.0,
                color: Color::rgb(0.9, 0.9, 0.9),
                ..default()
            },
        ));
    });

    let loading_ui = loading_ui_commands.id();

    let loading_commands = UiCommands::builder().with(UiCommand::SwapUi(loading_ui, ready_ui));

    loading_ui_commands.insert((
        Trigger::ScenesLoaded(scenes.into_iter().collect()),
        loading_commands,
    ));
}
