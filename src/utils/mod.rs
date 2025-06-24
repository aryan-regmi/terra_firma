use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

/// Stores all of the tilemaps for the game.
#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
pub struct Tilemaps(pub Vec<Tilemap>);

#[derive(Reflect, Default)]
pub struct Tilemap {
    pub name: Name,
    pub handle: Handle<TiledMap>,
    pub chunk_size: ChunkSize,
}

/// Represents a name.
#[derive(Component, Reflect, Default)]
pub struct Name(pub String);

#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
pub struct ChunkSize {
    pub width: f32,
    pub height: f32,
}
