use super::Player;
use crate::board::BoardPosition;
use bevy::{input::mouse::MouseMotion, prelude::*};
use std::f32::consts::PI;

#[derive(Component)]
pub struct FirstPersonPawn {
    camera: Entity,
    /// keep the camera this far above the player
    camera_height: f32,
    /// keep the camera this far behind the player
    camera_follow: f32,
    look_dist: f32,
    pub look_rotation: f32,
    /// limit the player's rotation to this speed per second
    turn_speed: f32,
    /// limit the player's movement speed per second
    pub speed: f32,
}

impl FirstPersonPawn {
    pub fn new(camera: Entity) -> FirstPersonPawn {
        FirstPersonPawn {
            camera,
            camera_height: 0.7,
            camera_follow: -0.25,
            look_dist: 10.0,
            look_rotation: 0.0,
            turn_speed: 0.05,
            speed: 10.0,
        }
    }
}

/// process input to move the player pawn
pub fn move_pawn(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut mouse_input: EventReader<MouseMotion>,
    mut pawns: Query<(&mut Transform, &mut FirstPersonPawn, Entity), With<Player>>,
    mut cameras: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let mouse_motion = mouse_input.read().fold(Vec2::ZERO, |a, d| a + d.delta);
    for (mut transform, mut pawn, entity) in pawns.iter_mut() {
        let mut delta = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::W) {
            delta += transform.forward();
        }
        if keyboard_input.pressed(KeyCode::A) {
            delta += transform.left();
        }
        if keyboard_input.pressed(KeyCode::S) {
            delta += transform.back();
        }
        if keyboard_input.pressed(KeyCode::D) {
            delta += transform.right();
        }
        delta.y = 0.0;
        let delta = delta.normalize_or_zero();
        transform.translation += delta * pawn.speed * time.delta_seconds();
        let motion_filtered = {
            let Vec2 { x, y } = mouse_motion;
            Vec2 {
                x: x.min(1000.0).max(-1000.0),
                y: y.min(1000.0).max(-1000.0),
            }
        };
        let motion =
            (motion_filtered * pawn.turn_speed) * time.delta_seconds() * Vec2::new(-1.0, -1.0);
        transform.rotate_local_y(motion.x);
        let Ok(mut camera_transform) = cameras.get_mut(pawn.camera) else {
            warn!("Pawn {entity:?}'s camera was not found");
            continue;
        };

        pawn.look_rotation = (pawn.look_rotation + motion.y).max(-0.5 * PI).min(0.5 * PI);

        let camera_destination = transform.translation
            + (transform.back() * pawn.camera_follow)
            + Vec3::new(0.0, pawn.camera_height, 0.0);
        // let motion =
        //     (camera_destination - camera_transform.translation) * 20.0 * time.delta_seconds();
        // camera_transform.translation += motion;
        camera_transform.translation = camera_destination;
        let camera_look = transform.translation + (transform.forward() * pawn.look_dist);
        camera_transform.look_at(camera_look, Vec3::Y);
        let axis = transform.right();
        camera_transform.rotate_axis(axis, pawn.look_rotation);
        // let look_delta = camera_look - camera_transform.translation;
        // let look_up = look_delta.cross(transform.left());
    }
}

pub fn update_board_position(
    mut pawns: Query<(&mut Transform, &mut BoardPosition), With<FirstPersonPawn>>,
) {
    for (transform, mut board_position) in pawns.iter_mut() {
        board_position.x = (transform.translation.x + 0.5).floor().max(0.0) as usize;
        board_position.y = (transform.translation.z + 0.5).floor().max(0.0) as usize;
    }
}
