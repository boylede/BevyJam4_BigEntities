#![allow(clippy::type_complexity)]
use assets::MyEmbeddedAssetsPlugin;
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_common_assets::ron::RonAssetPlugin;
use board::Board;
use pawn::PawnPlugin;
use pickup::PickupPlugin;
use states::{
    game_over::{display_score, gameover_keyboard},
    loading::setup_loading,
    menu::create_menu_ui,
    playing::{end_game, enter_menu, setup, teardown, update_scoreboard},
    GameState,
};
use ui::{button_clicked, trigger_check, fps::{setup_fps_counter, fps_text_update_system, fps_counter_showhide}};

pub mod assets;
pub mod board;
pub mod pawn;
pub mod pickup;
pub mod states;
pub mod ui;

/// global game state and loaded asset handles
#[derive(Resource)]
pub struct Game {
    player_handle: Handle<Scene>,
    pickup_handle: Handle<Scene>,
    tile_handle: Handle<Scene>,
    world_handle: Handle<Board>,
    score: i32,
    cake_eaten: u32,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MyEmbeddedAssetsPlugin,
            PawnPlugin,
            PickupPlugin,
            RonAssetPlugin::<Board>::new(&["world.ron"]),
        ))
        .add_state::<GameState>()
        .add_systems(OnEnter(GameState::Loading), setup_loading)
        .add_systems(Update, (trigger_check, button_clicked))
        .add_systems(OnEnter(GameState::Playing), setup)
        .add_systems(
            Update,
            (update_scoreboard, end_game, enter_menu).run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnExit(GameState::Playing), teardown)
        .add_systems(OnEnter(GameState::GameOver), display_score)
        .add_systems(OnEnter(GameState::Menu), create_menu_ui)
        .add_systems(
            Update,
            gameover_keyboard.run_if(in_state(GameState::GameOver)),
        )
        .add_systems(OnExit(GameState::GameOver), teardown);

        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
            app.add_systems(Startup, setup_fps_counter);
            app.add_systems(Update, (fps_text_update_system, fps_counter_showhide));
        }
        
    }
}
