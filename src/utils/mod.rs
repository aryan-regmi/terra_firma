use std::collections::HashMap;

use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

/// Stores all of the tilemaps for the game.
#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
pub struct Tilemaps(pub HashMap<Name, Tilemap>);

/// Stores tilemap information.
#[derive(Reflect, Default)]
pub struct Tilemap {
    pub handle: Handle<TiledMap>,
    pub chunk_size: ChunkSize,
}

/// A name `Component`.
#[derive(Component, Reflect, Default, PartialEq, Eq, Hash)]
pub struct Name(pub String);

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
pub struct ChunkSize {
    pub width: f32,
    pub height: f32,
}

/// Triggers when a map is loading.
#[derive(Event)]
pub struct SpawningMapEvent;

// NOTE: Start creating the player after this trigger
//
/// Triggers when a tilemap has been loaded.
#[derive(Event)]
pub struct MapSpawnedEvent;
