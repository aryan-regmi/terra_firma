pub mod animation;
pub mod button;
pub mod player;
pub mod screens;
pub mod tiled;

pub mod helper {
    use bevy::{ecs::system::SystemId, prelude::*};

    /// Component for a name.
    #[derive(Component, PartialEq, Eq, Default, Debug)]
    pub struct Name(pub(crate) String);

    #[derive(Resource)]
    pub struct CurrentMap(pub Name);

    #[allow(unused)]
    #[derive(Debug, Default)]
    pub struct Bounds {
        pub left: f32,
        pub right: f32,
        pub top: f32,
        pub bottom: f32,
    }

    #[derive(Debug, Default, Resource)]
    pub struct MapBounds(pub Bounds);

    #[derive(Debug, Resource)]
    pub struct CalculateBoundsId(pub SystemId);

    /// Claculates the bounds of the current map.
    pub(crate) fn calculate_bounds(
        current_map: Res<CurrentMap>,
        maps: Res<Assets<crate::tiled::TiledMap>>,
        window: Single<&Window>,
        tilemaps: Query<(&crate::helper::Name, &crate::tiled::TiledMapHandle)>,
        mut map_bounds: ResMut<MapBounds>,
    ) {
        let mut bounds = Bounds::default();
        for (name, tilemap) in tilemaps {
            if *name == current_map.0 {
                info_once!("Tiled Map Handle: {:?}", &tilemap.0);
                let tiled_map = maps.get(&tilemap.0);
                info_once!("Tiled Map: {:?}", tiled_map);
                if let Some(tiled_map) = tiled_map {
                    let (map_width, map_height) =
                        (tiled_map.map.width as f32, tiled_map.map.height as f32);
                    let (tile_width, tile_height) = (
                        tiled_map.map.tile_width as f32,
                        tiled_map.map.tile_height as f32,
                    );
                    let right_bound = map_width * tile_width;
                    let top_bound = map_height * tile_height;
                    let left_bound = -right_bound;
                    let bottom_bound = -top_bound;
                    bounds = Bounds {
                        left: left_bound,
                        right: right_bound,
                        top: top_bound,
                        bottom: bottom_bound,
                    };
                    break;
                } else {
                    let window_size = window.size();
                    bounds = Bounds {
                        left: -window_size.x,
                        right: window_size.x,
                        top: window_size.y,
                        bottom: -window_size.y,
                    };
                    break;
                }
            } else {
                let window_size = window.size();
                bounds = Bounds {
                    left: -window_size.x,
                    right: window_size.x,
                    top: window_size.y,
                    bottom: -window_size.y,
                };
                break;
            }
        }
        info_once!("Map bounds calculated: {:?}", bounds);
        map_bounds.0 = bounds;
    }
}

// TODO: Add egui inspector plugin!
//
// TODO: Add loader to load all assets in the beginning!
//  - Handle when asset isn't fully loaded
//
// TODO: Add animation using bevy_ecs_tilemap
//
// TODO: Add all `scale` constants as resources
//
// TODO: Replace external assests with custom ones!
//
// TODO: Make camera follow player
//
// TODO: Add `Zone { name: string }` property to tiles -> Adds a Zone component
//  - Can query for this component to change maps when a new zone is entered
//  - Setup maps the same way as screens
//
// TODO: Add `Zone { name: string }` property to tiles -> Adds a Zone component
//  - Can query for this component to change maps when a new zone is entered
//  - Setup maps the same way as screens
//
//
// TODO: Add `Animated { sprite_sheet, start_idx, end_idx, duration }` property to tiles -> flags
// for animation
//  - Add to the tiled plugin, so map tiles can be individually animated
