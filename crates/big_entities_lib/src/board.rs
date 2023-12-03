use bevy::prelude::*;

/// The position of this entity on its respective gameboard
#[derive(Component, Eq, PartialEq, Clone, Copy, Debug)]
pub struct BoardPosition {
    pub x: usize,
    pub y: usize,
}

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct Board {
    pub width: usize,
    pub height: usize,
}

/// Board tile
#[derive(Component)]
pub struct Cell {
    pub height: f32,
}

/// Parent of game board entities
#[derive(Component)]
pub struct BoardCache {
    width: usize,
    height: usize,
    inner: Vec<Tile>,
}

impl BoardCache {
    pub fn new(width: usize, height: usize, tiles: Vec<Tile>) -> BoardCache {
        BoardCache {
            width,
            height,
            inner: tiles,
        }
    }
    pub fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        let index = (y * self.width) + x;
        self.inner.get(index)
    }
    pub fn size(&self) -> (usize,usize) {
        (self.width, self.height)
    }
    /// the height of the tile, or 0.0 if tile not found
    pub fn get_height(&self, x: usize, y: usize) -> f32 {
        self.get(x, y).map(|t| t.height).unwrap_or(0.0)
    }
}

pub struct Tile {
    pub entity: Entity,
    pub height: f32,
}

pub const RESET_FOCUS: [f32; 3] = [
    2.0 as f32 / 2.0,
    0.0,
    2.0 as f32 / 2.0 - 0.5,
];
