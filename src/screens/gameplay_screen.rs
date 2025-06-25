use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_ecs_tiled::{map::TiledMapHandle, prelude::*};

use crate::{
    screens::{gameplay_screen::menu::PauseMenuPlugin, Screen},
    utils::{self, MapLoadedEvent, MapLoadingEvent, ResumeGameEvent},
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
        app.init_state::<GameState>();
        app.add_plugins(PauseMenuPlugin);
        app.add_systems(OnEnter(Screen::Gameplay), load_main_map)
            .add_systems(OnExit(Screen::Gameplay), unload_main_map)
            .add_systems(
                Update,
                pause_game.run_if(
                    in_state(Screen::Gameplay)
                        .and(in_state(GameState::Running))
                        .and(input_just_pressed(KeyCode::Escape)),
                ),
            );
        app.add_observer(map_load_observer);
        app.add_observer(resume_game_observer);
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
    info!("Game Paused");
}

/// Triggers `MapSpawnedEvent` when a tiled map has been loaded.
fn map_load_observer(_: Trigger<TiledMapCreated>, mut cmd: Commands) {
    info!("{} map loaded!", MAP_NAME);
    cmd.trigger(MapLoadedEvent);
}

/// Resumes the game when `ResumeGameEvent` is triggered.
fn resume_game_observer(_: Trigger<ResumeGameEvent>, mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Running);
    info!("Game resumed!");
}

mod menu {
    use bevy::prelude::*;
    use bevy_inspector_egui::{
        bevy_egui::{EguiContextPass, EguiContexts},
        egui,
    };

    use crate::{
        screens::{GameState, Screen},
        utils::ResumeGameEvent,
    };

    #[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
    #[states(scoped_entities)]
    enum PauseMenuState {
        #[default]
        Disabled,
        Main,
    }

    pub(crate) struct PauseMenuPlugin;

    impl Plugin for PauseMenuPlugin {
        fn build(&self, app: &mut App) {
            app.init_state::<PauseMenuState>();
            app.add_systems(
                EguiContextPass,
                display_menu.run_if(in_state(GameState::Paused)),
            );
            app.add_observer(close_menu_observer);
        }
    }

    fn display_menu(
        mut cmd: Commands,
        mut egui_ctxs: EguiContexts,
        mut menu_state: ResMut<NextState<PauseMenuState>>,
    ) {
        if let Some(ctx) = egui_ctxs.try_ctx_mut() {
            menu_state.set(PauseMenuState::Main);
            egui::Window::new("Paused").show(ctx, |ui| {
                if ui.button("Resume").clicked() {
                    cmd.trigger(ResumeGameEvent);
                }
            });
        }
    }

    /// Disables the menu when `ResumeGameEvent` is triggered.
    fn close_menu_observer(
        _: Trigger<ResumeGameEvent>,
        mut menu_state: ResMut<NextState<PauseMenuState>>,
    ) {
        menu_state.set(PauseMenuState::Disabled);
    }

    #[allow(unused)]
    /// Switches to the gameplay screen.
    fn switch_to_main_screen(mut screen: ResMut<NextState<Screen>>) {
        screen.set(Screen::Main);
    }
}
