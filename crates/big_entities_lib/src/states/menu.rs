use bevy::prelude::*;

use crate::ui::{UiCommand, UiCommands};

use super::GameState;

pub fn create_menu_ui(mut commands: Commands) {
    let mut ui_commands = commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        // visibility: Visibility::Hidden,
        ..default()
    });

    let ui_id = ui_commands.id();
    let continue_button_commands = UiCommands::builder()
        .with(UiCommand::SwitchState(GameState::Playing))
        .with(UiCommand::CaptureMouse)
        .with(UiCommand::Despawn(ui_id));

    let exit_button_commands = UiCommands::builder()
        .with(UiCommand::SwitchState(GameState::GameOver))
        .with(UiCommand::ReleaseMouse)
        .with(UiCommand::Despawn(ui_id))
        .with(UiCommand::Terminate);

    ui_commands.with_children(|parent| {
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
            .insert(continue_button_commands)
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Continue",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ));
            });
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
            .insert(exit_button_commands)
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Exit",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ));
            });
    });
}
