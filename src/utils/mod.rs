use std::collections::HashMap;

use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use bevy_inspector_egui::egui;

/// Stores all of the tilemaps for the game.
#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
pub(crate) struct Tilemaps(pub(crate) HashMap<Name, Tilemap>);

/// Stores tilemap information.
#[derive(Reflect, Default)]
pub(crate) struct Tilemap {
    pub(crate) handle: Handle<TiledMap>,
    pub(crate) chunk_size: ChunkSize,
}

/// A name `Component`.
#[derive(Component, Reflect, Default, PartialEq, Eq, Hash)]
pub(crate) struct Name(pub(crate) String);

impl Into<String> for Name {
    fn into(self) -> String {
        self.0
    }
}

impl From<&str> for Name {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

/// The chunk size (in tiles) of a tilemap.
#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
pub(crate) struct ChunkSize {
    pub(crate) width: f32,
    pub(crate) height: f32,
}

/// Triggers when a map is loading.
#[derive(Event)]
pub(crate) struct MapLoadingEvent;

// NOTE: Start creating the player after this trigger
//
/// Triggers when a tilemap has been loaded.
#[derive(Event)]
pub(crate) struct MapLoadedEvent;

/// Triggers when the game has been resumed.
#[derive(Event)]
pub(crate) struct ResumeGameEvent;

/// Loads the specified map.
pub(crate) fn load_map(
    map_name: &str,
    map_path: &str,
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    mut maps: ResMut<Tilemaps>,
) {
    cmd.trigger(MapLoadingEvent);
    info!("Loading {} map...", map_name);
    if let Some(tilemap) = maps.0.get(&map_name.into()) {
        // Retrieve handle and spawn a new map entity
        cmd.spawn((
            Name(map_name.into()),
            TiledMapHandle(tilemap.handle.clone_weak()),
            TilemapAnchor::Center,
            TiledWorldChunking::new(tilemap.chunk_size.width, tilemap.chunk_size.height),
            TiledMapMarker,
        ));
    } else {
        // Load a map asset and retrieve the corresponding handle
        let tilemap = Tilemap {
            handle: asset_server.load(map_path),
            chunk_size: ChunkSize {
                width: 256.0,
                height: 256.0,
            },
        };

        // Spawn a new entity with the newly created handle
        cmd.spawn((
            Name(map_name.into()),
            TiledMapHandle(tilemap.handle.clone_weak()),
            TilemapAnchor::Center,
            TiledWorldChunking::new(tilemap.chunk_size.width, tilemap.chunk_size.height),
        ));
        maps.0.insert(map_name.into(), tilemap);
    }
}

/// Creates a button with the given label and size.
pub(crate) fn sized_button(
    ui: &mut egui::Ui,
    label: &str,
    width: f32,
    height: f32,
) -> egui::Response {
    ui.add_sized((width, height), egui::Button::new(label))
}
