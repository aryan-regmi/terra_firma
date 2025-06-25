use std::collections::HashMap;

use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

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
