use bevy::{input::mouse::MouseWheel, prelude::*};

/// Animate camera to follow entity
#[derive(Component)]
pub struct FollowMe {
    pub camera: Entity,
    pub zoom: f32,
    pub offset: Vec3,
    // rotation: Quat,
}

impl FollowMe {
    pub fn new(camera: Entity) -> FollowMe {
        FollowMe {
            camera,
            zoom: 10.0,
            /// offset from the pawn's location to look at
            offset: Vec3::ZERO,
            // rotation: Quat::from_rotation_x(0.0),
        }
    }
}

pub fn update_follow_camera(
    time: Res<Time>,
    mut cameras: Query<&mut Transform, With<Camera>>,
    follow: Query<(&Transform, &FollowMe), Without<Camera>>,
) {
    for (
        transform,
        FollowMe {
            camera,
            zoom,
            offset,
        },
    ) in follow.iter()
    {
        let Ok(mut camera_transform) = cameras.get_mut(*camera) else {
            warn!("camera didn't exist");
            return;
        };
        let desired_position = {
            let above = Vec3::new(0.0, *zoom, 0.0);
            let camera_baseline = transform.translation + (transform.back() * *zoom) + above;
            // Transform::from_translation(camera_baseline).looking_at(transform.translation, Vec3::Z)
            camera_baseline
        };
        let mut motion = desired_position - camera_transform.translation;
        if motion.length().abs() > 0.02 {
            motion *= 2.0 * time.delta_seconds();
        }
        camera_transform.translation += motion;
        camera_transform.look_at(transform.translation + *offset, Vec3::Y);
    }
}

pub fn camera_follow_zoom(mut wheel: EventReader<MouseWheel>, mut follow: Query<&mut FollowMe>) {
    for MouseWheel { unit, y, .. } in wheel.read() {
        use bevy::input::mouse::MouseScrollUnit as MSU;
        let scroll = match unit {
            MSU::Line => *y * 8.0,
            MSU::Pixel => *y,
        };
        for mut follow in follow.iter_mut() {
            if follow.zoom >= 3.0 && follow.zoom <= 100.0 {
                follow.zoom += scroll / 10.0;
            }
            if follow.zoom < 3.0 {
                follow.zoom = 3.0;
            } else if follow.zoom > 100.0 {
                follow.zoom = 100.0;
            }
        }
    }
}
