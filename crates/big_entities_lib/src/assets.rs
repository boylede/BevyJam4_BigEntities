use bevy::{asset::embedded_asset, prelude::*};

/// embedded assets
pub struct MyEmbeddedAssetsPlugin;

impl Plugin for MyEmbeddedAssetsPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "assets/models/AlienCake/tile.glb"); // #Scene0
        embedded_asset!(app, "assets/models/AlienCake/alien.glb");
        embedded_asset!(app, "assets/models/AlienCake/cakeBirthday.glb");
        // embedded_asset!(app, "assets/worlds/main.world.ron");
    }
}
