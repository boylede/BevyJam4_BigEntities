use bevy::ecs::{bundle::Bundle, component::Component};

use super::{InputRateLimit, Player};

/// the components needed for use with the third person camera following
#[derive(Bundle)]
pub struct ThirdPersonPawnBundle {
    player: Player,
    facing: Facing,
    input_rate_limit: InputRateLimit,
}

#[derive(Component, Copy, Clone, Debug)]
pub enum Facing {
    Up,
    Down,
    Right,
    Left,
}
