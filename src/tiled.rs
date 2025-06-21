use std::io::{Cursor, ErrorKind};
use std::path::Path;
use std::sync::Arc;

use avian2d::prelude::*;
use bevy::log::{info, warn};
use bevy::math::Vec3;
use bevy::{
    asset::{io::Reader, AssetLoader, AssetPath},
    platform::collections::HashMap,
    prelude::{
        Added, Asset, AssetApp, AssetEvent, AssetId, Assets, Bundle, Commands, Component, Entity,
        EventReader, GlobalTransform, Handle, Image, Plugin, Query, Res, Transform, Update,
    },
    reflect::TypePath,
};
use bevy_ecs_tilemap::prelude::*;

const MAP_SCALE: f32 = 2.0;

/// A marker component for objects that can be collided with.
#[derive(Component, Default, Debug)]
pub struct TiledColliderObject;

#[derive(Default)]
pub struct TiledMapPlugin;

impl Plugin for TiledMapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_asset::<TiledMap>()
            .register_asset_loader(TiledLoader)
            .add_systems(Update, process_loaded_maps);
    }
}

#[derive(TypePath, Asset)]
pub struct TiledMap {
    pub map: tiled::Map,

    pub tilemap_textures: HashMap<usize, TilemapTexture>,
}

// Stores a list of tiled layers.
#[derive(Component, Default)]
pub struct TiledLayersStorage {
    pub storage: HashMap<u32, Entity>,
}

#[derive(Component, Default)]
pub struct TiledMapHandle(pub Handle<TiledMap>);

#[derive(Default, Bundle)]
pub struct TiledMapBundle {
    pub tiled_map: TiledMapHandle,
    pub storage: TiledLayersStorage,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub render_settings: TilemapRenderSettings,
}

struct BytesResourceReader {
    bytes: Arc<[u8]>,
}

impl BytesResourceReader {
    fn new(bytes: &[u8]) -> Self {
        Self {
            bytes: Arc::from(bytes),
        }
    }
}

impl tiled::ResourceReader for BytesResourceReader {
    type Resource = Cursor<Arc<[u8]>>;
    type Error = std::io::Error;

    fn read_from(&mut self, _path: &Path) -> std::result::Result<Self::Resource, Self::Error> {
        // In this case, the path is ignored because the byte data is already provided.
        Ok(Cursor::new(self.bytes.clone()))
    }
}

pub struct TiledLoader;

impl AssetLoader for TiledLoader {
    type Asset = TiledMap;
    type Settings = ();
    type Error = String;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader
            .read_to_end(&mut bytes)
            .await
            .map_err(|e| e.to_string())?;

        let mut loader = tiled::Loader::with_cache_and_reader(
            tiled::DefaultResourceCache::new(),
            BytesResourceReader::new(&bytes),
        );

        let map = loader
            .load_tmx_map(load_context.path())
            .map_err(|e| {
                std::io::Error::new(ErrorKind::Other, format!("Could not load TMX map: {e}"))
            })
            .map_err(|e| e.to_string())?;

        let mut tilemap_textures = HashMap::default();

        for (tileset_index, tileset) in map.tilesets().iter().enumerate() {
            let tilemap_texture = match &tileset.image {
                None => {
                    info!(
                            "Skipping image collection tileset '{}' which is incompatible with atlas feature",
                            tileset.name
                        );
                    continue;
                }

                Some(img) => {
                    // The load context path is the TMX file itself. If the file is at the root of the
                    // assets/ directory structure then the tmx_dir will be empty, which is fine.
                    let tile_path = img
                        .source
                        .to_str()
                        .expect("The asset load context was empty.");
                    let asset_path = AssetPath::from(tile_path);
                    let texture: Handle<Image> = load_context.load(asset_path.clone());

                    TilemapTexture::Single(texture.clone())
                }
            };

            tilemap_textures.insert(tileset_index, tilemap_texture);
        }

        let asset_map = TiledMap {
            map,
            tilemap_textures,
        };

        info!("Loaded map: {}", load_context.path().display());
        Ok(asset_map)
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["tmx"];
        EXTENSIONS
    }
}

pub fn process_loaded_maps(
    mut commands: Commands,
    mut map_events: EventReader<AssetEvent<TiledMap>>,
    maps: Res<Assets<TiledMap>>,
    tile_storage_query: Query<(Entity, &TileStorage)>,
    mut map_query: Query<(
        &TiledMapHandle,
        &mut TiledLayersStorage,
        &TilemapRenderSettings,
    )>,
    new_maps: Query<&TiledMapHandle, Added<TiledMapHandle>>,
) {
    let mut changed_maps = Vec::<AssetId<TiledMap>>::default();
    for event in map_events.read() {
        match event {
            AssetEvent::Added { id } => {
                info!("Map added!");
                changed_maps.push(*id);
            }
            AssetEvent::Modified { id } => {
                info!("Map changed!");
                changed_maps.push(*id);
            }
            AssetEvent::Removed { id } => {
                info!("Map removed!");
                // if mesh was modified and removed in the same update, ignore the modification
                // events are ordered so future modification events are ok
                changed_maps.retain(|changed_handle| changed_handle == id);
            }
            _ => continue,
        }
    }

    // If we have new map entities add them to the changed_maps list.
    for new_map_handle in new_maps.iter() {
        changed_maps.push(new_map_handle.0.id());
    }

    for changed_map in changed_maps.iter() {
        for (map_handle, mut layer_storage, render_settings) in map_query.iter_mut() {
            // only deal with currently changed map
            if map_handle.0.id() != *changed_map {
                continue;
            }
            if let Some(tiled_map) = maps.get(&map_handle.0) {
                // TODO: Create a RemoveMap component..
                for layer_entity in layer_storage.storage.values() {
                    if let Ok((_, layer_tile_storage)) = tile_storage_query.get(*layer_entity) {
                        for tile in layer_tile_storage.iter().flatten() {
                            commands.entity(*tile).despawn()
                        }
                    }
                }

                // The TilemapBundle requires that all tile images come exclusively from a single
                // tiled texture or from a Vec of independent per-tile images. Furthermore, all of
                // the per-tile images must be the same size. Since Tiled allows tiles of mixed
                // tilesets on each layer and allows differently-sized tile images in each tileset,
                // this means we need to load each combination of tileset and layer separately.
                for (tileset_index, tileset) in tiled_map.map.tilesets().iter().enumerate() {
                    let Some(tilemap_texture) = tiled_map.tilemap_textures.get(&tileset_index)
                    else {
                        warn!("Skipped creating layer with missing tilemap textures.");
                        continue;
                    };

                    let tile_size = TilemapTileSize {
                        x: tileset.tile_width as f32,
                        y: tileset.tile_height as f32,
                    };

                    let tile_spacing = TilemapSpacing {
                        x: tileset.spacing as f32,
                        y: tileset.spacing as f32,
                    };

                    // Once materials have been created/added we need to then create the layers.
                    for (layer_index, layer) in tiled_map.map.layers().enumerate() {
                        let offset_x = layer.offset_x;
                        let offset_y = layer.offset_y;

                        let layer_data = {
                            match layer.layer_type() {
                                tiled::LayerType::Tiles(tile_layer) => {
                                    if let tiled::TileLayer::Finite(layer_data) = tile_layer {
                                        (Some(layer_data), None)
                                    } else {
                                        info!(
                                    "Skipping layer {} because only finite layers are supported.",
                                    layer.id()
                                );
                                        continue;
                                    }
                                }

                                tiled::LayerType::Objects(object_layer) => {
                                    (None, Some(object_layer))
                                }

                                _ => {
                                    info!(
                                "Skipping layer {} because only tile and object layers are supported.",
                                layer.id()
                            );
                                    continue;
                                }
                            }
                        };

                        let map_size = TilemapSize {
                            x: tiled_map.map.width,
                            y: tiled_map.map.height,
                        };

                        let grid_size = TilemapGridSize {
                            x: tiled_map.map.tile_width as f32,
                            y: tiled_map.map.tile_height as f32,
                        };

                        let map_type = match tiled_map.map.orientation {
                            tiled::Orientation::Hexagonal => {
                                TilemapType::Hexagon(HexCoordSystem::Row)
                            }
                            tiled::Orientation::Isometric => {
                                TilemapType::Isometric(IsoCoordSystem::Diamond)
                            }
                            tiled::Orientation::Staggered => {
                                TilemapType::Isometric(IsoCoordSystem::Staggered)
                            }
                            tiled::Orientation::Orthogonal => TilemapType::Square,
                        };

                        let mut tile_storage = TileStorage::empty(map_size);
                        let layer_entity = commands.spawn_empty().id();

                        for x in 0..map_size.x {
                            for y in 0..map_size.y {
                                // Transform TMX coords into bevy coords.
                                let mapped_y = tiled_map.map.height - 1 - y;

                                let mapped_x = x as i32;
                                let mapped_y = mapped_y as i32;

                                // Handles the tile layer
                                let mut handle_tile_layer = |layer_data: tiled::FiniteTileLayer<
                                    '_,
                                >| {
                                    if let Some(layer_tile) =
                                        layer_data.get_tile(mapped_x, mapped_y)
                                    {
                                        if tileset_index != layer_tile.tileset_index() {
                                            return false;
                                        }

                                        let layer_tile_data =
                                            match layer_data.get_tile_data(mapped_x, mapped_y) {
                                                Some(d) => d,
                                                None => {
                                                    return false;
                                                }
                                            };

                                        let texture_index = match tilemap_texture {
                                            TilemapTexture::Single(_) => layer_tile.id(),
                                            _ => unreachable!(),
                                        };

                                        let tile_pos = TilePos { x, y };

                                        let tile_entity = commands
                                            .spawn(TileBundle {
                                                position: tile_pos,
                                                tilemap_id: TilemapId(layer_entity),
                                                texture_index: TileTextureIndex(texture_index),
                                                flip: TileFlip {
                                                    x: layer_tile_data.flip_h,
                                                    y: layer_tile_data.flip_v,
                                                    d: layer_tile_data.flip_d,
                                                },
                                                ..Default::default()
                                            })
                                            .id();

                                        tile_storage.set(&tile_pos, tile_entity);
                                        true
                                    } else {
                                        true
                                    }
                                };

                                // Handles the object layer
                                let handle_object_layer = |object_data: &tiled::ObjectData| {
                                    let object_tile_data = match object_data.tile_data() {
                                        Some(d) => d,
                                        None => {
                                            return None;
                                        }
                                    };

                                    let texture_index = match tilemap_texture {
                                        TilemapTexture::Single(_) => object_tile_data.id(),
                                        _ => unreachable!(),
                                    };

                                    // Transform TMX coords into bevy coords.
                                    let (x, y) = {
                                        let x = (object_data.x / tiled_map.map.width as f32) as u32;
                                        let y =
                                            (object_data.y / tiled_map.map.height as f32) as u32;
                                        (x, y)
                                    };
                                    let tile_pos = TilePos { x, y };

                                    let collider_type = if let Some(collider_type_value) =
                                        object_data.properties.get("collider_type")
                                    {
                                        if let tiled::PropertyValue::StringValue(
                                            collider_type_string,
                                        ) = collider_type_value
                                        {
                                            Some(match collider_type_string.as_str() {
                                                "Dynamic" => RigidBody::Dynamic,
                                                "Static" => RigidBody::Static,
                                                "Kinematic" => RigidBody::Kinematic,
                                                _ => unreachable!(),
                                            })
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    };

                                    let hitbox = {
                                        let raw_hitbox = if let Some(hitbox_value) =
                                            object_data.properties.get("hitbox")
                                        {
                                            if let tiled::PropertyValue::ClassValue {
                                                properties,
                                                ..
                                            } = hitbox_value
                                            {
                                                let width = if let Some(width) =
                                                    properties.get("width")
                                                {
                                                    if let tiled::PropertyValue::FloatValue(width) =
                                                        width
                                                    {
                                                        Some(width)
                                                    } else {
                                                        None
                                                    }
                                                } else {
                                                    None
                                                };

                                                let height = if let Some(height) =
                                                    properties.get("height")
                                                {
                                                    if let tiled::PropertyValue::FloatValue(
                                                        height,
                                                    ) = height
                                                    {
                                                        Some(height)
                                                    } else {
                                                        None
                                                    }
                                                } else {
                                                    None
                                                };

                                                (width, height)
                                            } else {
                                                (None, None)
                                            }
                                        } else {
                                            (None, None)
                                        };

                                        let (width, height) = raw_hitbox;
                                        if width.is_some() && height.is_some() {
                                            (width.unwrap(), height.unwrap())
                                        } else if width.is_some() && height.is_none() {
                                            (width.unwrap(), width.unwrap())
                                        } else if width.is_none() && height.is_some() {
                                            (height.unwrap(), height.unwrap())
                                        } else {
                                            (
                                                &(tileset.tile_width as f32),
                                                &(tileset.tile_height as f32),
                                            )
                                        }
                                    };

                                    let tile_entity = (
                                        TileBundle {
                                            position: tile_pos,
                                            tilemap_id: TilemapId(layer_entity),
                                            texture_index: TileTextureIndex(texture_index),
                                            flip: TileFlip {
                                                x: object_tile_data.flip_h,
                                                y: object_tile_data.flip_v,
                                                d: object_tile_data.flip_d,
                                            },
                                            ..Default::default()
                                        },
                                        TiledColliderObject,
                                        collider_type.unwrap_or_else(|| RigidBody::Static),
                                        Collider::rectangle(*hitbox.0, *hitbox.1),
                                    );

                                    Some((tile_pos, tile_entity))
                                };

                                match layer_data {
                                    (Some(tile_layer), None) => {
                                        if !handle_tile_layer(tile_layer) {
                                            continue;
                                        }
                                    }

                                    (None, Some(object_layer)) => {
                                        let objects = object_layer.object_data();
                                        for object_data in objects {
                                            if let Some((tile_pos, tile_entity)) =
                                                handle_object_layer(object_data)
                                            {
                                                let tile_entity = commands.spawn(tile_entity).id();
                                                tile_storage.set(&tile_pos, tile_entity);
                                            }
                                        }
                                    }

                                    (Some(tile_layer), Some(object_layer)) => {
                                        // Handle tile layer
                                        if !handle_tile_layer(tile_layer) {
                                            continue;
                                        }

                                        // Handle object layer
                                        {
                                            let objects = object_layer.object_data();
                                            for object_data in objects {
                                                if let Some((tile_pos, tile_entity)) =
                                                    handle_object_layer(object_data)
                                                {
                                                    let tile_entity =
                                                        commands.spawn(tile_entity).id();
                                                    tile_storage.set(&tile_pos, tile_entity);
                                                }
                                            }
                                        }
                                    }

                                    (None, None) => continue,
                                }
                            }
                        }

                        commands.entity(layer_entity).insert(TilemapBundle {
                            grid_size,
                            size: map_size,
                            storage: tile_storage,
                            texture: tilemap_texture.clone(),
                            tile_size,
                            spacing: tile_spacing,
                            anchor: TilemapAnchor::Center,
                            transform: Transform::from_xyz(offset_x, -offset_y, layer_index as f32)
                                .with_scale(Vec3::splat(MAP_SCALE)),
                            map_type,
                            render_settings: *render_settings,
                            ..Default::default()
                        });

                        layer_storage
                            .storage
                            .insert(layer_index as u32, layer_entity);
                    }
                }
            }
        }
    }
}
