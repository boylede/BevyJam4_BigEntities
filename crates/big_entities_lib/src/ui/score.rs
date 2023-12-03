use bevy::prelude::*;

#[derive(Component)]
pub enum DataDisplay {
    Score,
    PickupPosition,
    PlayerPosition,
}
