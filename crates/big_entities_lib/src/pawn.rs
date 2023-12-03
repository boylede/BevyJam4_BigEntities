use crate::states::GameState;

use self::{
    board_movement::move_pawn_board_position,
    follow_camera::{camera_follow_zoom, update_follow_camera},
    free_movement::{move_pawn, update_board_position},
};
use bevy::ecs::component::Component;
use bevy::prelude::*;

pub mod board_movement;
pub mod follow_camera;
pub mod free_movement;
pub mod third_person;

pub struct PawnPlugin;

impl Plugin for PawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                move_pawn_board_position,
                move_pawn,
                update_follow_camera,
                camera_follow_zoom,
                update_board_position,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

/// Marker for players
#[derive(Component)]
pub struct Player;

/// Entities with this component will only process input when this timer is expired
#[derive(Component)]
pub struct InputRateLimit(pub Timer);
