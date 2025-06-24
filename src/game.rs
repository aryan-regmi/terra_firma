use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

use crate::utils;

/// Manages resources and systems for the entire game.
#[derive(Debug, Default)]
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<utils::Tilemaps>()
            .register_type::<utils::Tilemaps>()
            .register_type::<utils::ChunkSize>()
            .add_plugins(TiledMapPlugin::default())
            .add_systems(Startup, startup);
    }
}

fn startup(mut cmd: Commands, asset_server: Res<AssetServer>, mut maps: ResMut<utils::Tilemaps>) {
    // Spawn a Bevy 2D camera
    cmd.spawn(Camera2d);

    // Load a map asset and retrieve the corresponding handle
    let tilemap = utils::Tilemap {
        name: utils::Name("Main".into()),
        handle: asset_server.load("maps/map_00/main.tmx"),
        chunk_size: utils::ChunkSize {
            width: 32.0,
            height: 32.0,
        },
    };

    // Spawn a new entity with this handle
    cmd.spawn((
        TiledMapHandle(tilemap.handle.clone_weak()),
        TilemapAnchor::Center,
        TiledWorldChunking::new(tilemap.chunk_size.width, tilemap.chunk_size.height),
    ));
    maps.0.push(tilemap);
}
