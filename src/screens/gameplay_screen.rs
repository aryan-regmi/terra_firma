use bevy::{input::common_conditions::input_pressed, prelude::*};
use bevy_ecs_tiled::{map::TiledMapHandle, prelude::*};

use crate::{
    screens::{gameplay_screen::menu::MenuPlugin, Screen},
    utils::{self, MapLoadedEvent, MapLoadingEvent},
};

const MAP_NAME: &str = "Main";

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
#[states(scoped_entities)]
pub(crate) enum GameState {
    #[default]
    Running,
    Paused,
}

pub(crate) struct GameplayScreenPlugin;

impl Plugin for GameplayScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Gameplay), load_main_map)
            .add_systems(OnExit(Screen::Gameplay), unload_main_map)
            .add_systems(
                Update,
                pause_game.run_if(in_state(Screen::Gameplay).and(input_pressed(KeyCode::Escape))),
            );
        app.add_plugins(MenuPlugin);
        app.add_observer(map_load_observer);
    }
}

fn load_main_map(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    mut maps: ResMut<utils::Tilemaps>,
) {
    cmd.trigger(MapLoadingEvent);
    info!("Loading {} map...", MAP_NAME);
    if let Some(tilemap) = maps.0.get(&MAP_NAME.into()) {
        // Retrieve handle and spawn a new map entity
        cmd.spawn((
            utils::Name(MAP_NAME.into()),
            TiledMapHandle(tilemap.handle.clone_weak()),
            TilemapAnchor::Center,
            TiledWorldChunking::new(tilemap.chunk_size.width, tilemap.chunk_size.height),
            TiledMapMarker,
        ));
    } else {
        // Load a map asset and retrieve the corresponding handle
        let tilemap = utils::Tilemap {
            handle: asset_server.load("maps/map_00/main.tmx"),
            chunk_size: utils::ChunkSize {
                width: 256.0,
                height: 256.0,
            },
        };

        // Spawn a new entity with the newly created handle
        cmd.spawn((
            utils::Name(MAP_NAME.into()),
            TiledMapHandle(tilemap.handle.clone_weak()),
            TilemapAnchor::Center,
            TiledWorldChunking::new(tilemap.chunk_size.width, tilemap.chunk_size.height),
        ));
        maps.0.insert(MAP_NAME.into(), tilemap);
    }
}

/// Unloads the main map.
///
/// # Note
/// This does **NOT** remove the map from the `Tilemaps` resources.
fn unload_main_map(mut cmd: Commands, maps: Query<(Entity, &utils::Name), With<TiledMapMarker>>) {
    for (map, name) in maps {
        if name.0 == "Main" {
            cmd.entity(map).despawn();
        }
    }
}

/// Pauses the game.
fn pause_game(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Paused);
}

/// Triggers `MapSpawnedEvent` when a tiled map has been loaded.
fn map_load_observer(_: Trigger<TiledMapCreated>, mut cmd: Commands) {
    info!("{} map loaded!", MAP_NAME);
    cmd.trigger(MapLoadedEvent);
}

mod menu {
    use bevy::prelude::*;

    use crate::screens::{GameState, Screen};

    #[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
    #[states(scoped_entities)]
    enum MenuState {
        #[default]
        Disabled,
        Main,
    }

    pub(crate) struct MenuPlugin;

    impl Plugin for MenuPlugin {
        fn build(&self, app: &mut App) {
            app.init_state::<GameState>();
            app.add_systems(OnEnter(GameState::Paused), menu_setup);
        }
    }

    fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
        menu_state.set(MenuState::Main);
    }

    /// Switches to the gameplay screen.
    fn switch_to_main_screen(mut screen: ResMut<NextState<Screen>>) {
        screen.set(Screen::Main);
    }
}
