use super::GameState;
use crate::Game;
use bevy::prelude::*;

/// process keyboard input during game over state
pub fn gameover_keyboard(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
}

/// add score display to screen during gameend
pub fn display_score(mut commands: Commands, game: Res<Game>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                format!("Cake eaten: {}", game.cake_eaten),
                TextStyle {
                    font_size: 80.0,
                    color: Color::rgb(0.5, 0.5, 1.0),
                    ..default()
                },
            ));
        });
}
