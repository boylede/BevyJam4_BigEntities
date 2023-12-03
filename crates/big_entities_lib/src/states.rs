use bevy::prelude::*;

pub mod game_over;
pub mod loading;
pub mod menu;
pub mod playing;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    Loading,
    Playing,
    GameOver,
    Menu,
}
