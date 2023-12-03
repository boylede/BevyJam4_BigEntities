use bevy::{asset::embedded_asset, prelude::*};

/// embedded assets
pub struct MyEmbeddedAssetsPlugin;

impl Plugin for MyEmbeddedAssetsPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "assets/models/AlienCake/tile.glb"); // #Scene0
        embedded_asset!(app, "assets/models/AlienCake/alien.glb");
        embedded_asset!(app, "assets/models/AlienCake/cakeBirthday.glb");
        embedded_asset!(app, "assets/worlds/main.world.ron");
        // {
        //     {
        //         let embedded = app
        //             .world
        //             .resource_mut::<bevy::asset::io::embedded::EmbeddedAssetRegistry>();
        //         let path = {
        //             let crate_name = module_path!().split(':').next().unwrap();
        //             info!("crate name: {crate_name}");
        //             let after_src = file!().split("/src/").nth(1).unwrap();
        //             info!("after src: {after_src}");
        //             let file_path = std::path::Path::new(after_src)
        //                 .parent()
        //                 .unwrap()
        //                 .join("assets/models/AlienCake/alien.glb");
        //             info!("file_path: {file_path:?}");
        //             std::path::Path::new(crate_name).join(file_path)
        //         };
        //         info!("path: {path:?}");
        //         let full_path = std::path::PathBuf::new();
        //         embedded.insert_asset(
        //             full_path,
        //             &path,
        //             include_bytes!("../../../assets/models/AlienCake/alien.glb"),
        //         );
        //     }
        // };
    }
}
