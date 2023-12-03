use crate::{
    board::{Board, BoardCache, BoardPosition},
    pawn::third_person::Facing,
    Game,
};
use bevy::prelude::*;
use std::f32::consts::PI;

use super::{InputRateLimit, Player};

/// process input to move the player pawn
pub fn move_pawn_board_position(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<
        (
            &mut Transform,
            &mut BoardPosition,
            &mut InputRateLimit,
            &mut Facing,
        ),
        With<Player>,
    >,
    gameboard: Query<&BoardCache>,
    game: Res<Game>,
    boards: Res<Assets<Board>>,
) {
    let Some(board_config) = boards.get(game.world_handle.clone()) else {
        warn!("expected board config asset to be loaded by now");
        return;
    };
    let Ok(board_cache) = gameboard.get_single() else {
        warn!("unexpected number of game boards");
        return;
    };
    for (mut t, mut player, mut irl, mut facing) in players.iter_mut() {
        if !irl.0.tick(time.delta()).finished() {
            continue;
        }

        type Direction = (i32, i32, f32, Facing);
        use Facing as F;
        let directions: [Direction; 4] = {
            const X_POS: Direction = (1, 0, -PI / 2.0, F::Up);
            const X_NEG: Direction = (-1, 0, PI / 2.0, F::Down);
            const Y_POS: Direction = (0, 1, PI, F::Left);
            const Y_NEG: Direction = (0, -1, 0.0, F::Right);
            match *facing {
                F::Up => [X_POS, X_NEG, Y_POS, Y_NEG],
                F::Down => [X_NEG, X_POS, Y_NEG, Y_POS],
                F::Right => [Y_NEG, Y_POS, X_POS, X_NEG],
                F::Left => [Y_POS, Y_NEG, X_NEG, X_POS],
            }
        };

        if let Some((x_delta, y_delta, rotation, face)) = {
            if keyboard_input.pressed(KeyCode::Up) {
                Some(directions[0])
            } else if keyboard_input.pressed(KeyCode::Down) {
                Some(directions[1])
            } else if keyboard_input.pressed(KeyCode::Right) {
                Some(directions[2])
            } else if keyboard_input.pressed(KeyCode::Left) {
                Some(directions[3])
            } else {
                None
            }
        } {
            *facing = face;
            player.x = (player.x as i32 + x_delta)
                .min(board_config.width as i32 - 1)
                .max(0) as usize;
            player.y = (player.y as i32 + y_delta)
                .min(board_config.height as i32 - 1)
                .max(0) as usize;
            irl.0.reset();
            let new_transform = Transform {
                translation: Vec3::new(
                    player.x as f32,
                    board_cache.get_height(player.x, player.y),
                    player.y as f32,
                ),
                rotation: Quat::from_rotation_y(rotation),
                ..default()
            };
            *t = new_transform;
        }
    }
}
