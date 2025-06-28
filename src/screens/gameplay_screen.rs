use std::collections::HashMap;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tilemap::map::TilemapTexture;

use crate::{
    player,
    screens::{gameplay_screen::menu::PauseMenuPlugin, Screen},
    utils::{self, MapLoadedEvent, MapLoadingEvent, ResumeGameEvent, ReturnToMainMenuEvent},
};

const MAP_NAME: &str = "Main";

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
#[states(scoped_entities)]
pub(crate) enum GameState {
    #[default]
    Started,
    Running,
    Paused,
    Exit,
}

pub(crate) struct GameplayScreenPlugin;

impl Plugin for GameplayScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        app.init_resource::<OriginalTextureColors>();
        app.add_plugins((PauseMenuPlugin, player::PlayerPlugin));
        app.add_systems(
            OnEnter(Screen::Gameplay),
            (load_main_map, setup_default_zoom).chain(),
        )
        .add_systems(OnExit(Screen::Gameplay), unload_main_map)
        .add_systems(
            Update,
            pause_game.run_if(
                in_state(Screen::Gameplay)
                    .and(in_state(GameState::Running))
                    .and(input_just_pressed(KeyCode::Escape)),
            ),
        )
        .add_systems(OnEnter(GameState::Paused), grey_out_game)
        .add_systems(OnExit(GameState::Paused), undo_greyed_out_game);
        app.add_observer(map_load_observer);
        app.add_observer(resume_game_observer);
        app.add_observer(switch_to_main_screen_observer);
    }
}

/// The factor to scale the zoom by.
const DEFAULT_ZOOM_FACTOR: f32 = 0.5;

fn setup_default_zoom(mut projection: Single<&mut Projection>) {
    match &mut **projection {
        Projection::Orthographic(orthographic_projection) => {
            orthographic_projection.scale = DEFAULT_ZOOM_FACTOR;
        }
        _ => error!("Unhandled projection type"),
    }
}

/// Loads the main map
fn load_main_map(mut cmd: Commands, asset_server: Res<AssetServer>, maps: ResMut<utils::Tilemaps>) {
    cmd.trigger(MapLoadingEvent);
    utils::load_map(MAP_NAME, "maps/map_00/main.tmx", cmd, asset_server, maps);
}

/// Unloads the main map.
///
/// # Note
/// This does **NOT** remove the map from the `Tilemaps` resources.
fn unload_main_map(mut cmd: Commands, maps: Query<(Entity, &utils::Name), With<TiledMapMarker>>) {
    for (map, name) in maps {
        if name.0 == MAP_NAME {
            cmd.entity(map).despawn();
            info!("{} map unloaded!", MAP_NAME);
        }
    }
}

/// Pauses the game.
fn pause_game(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Paused);
    info!("Game Paused");
}

#[derive(Default, Resource)]
struct OriginalTextureColors(HashMap<(Handle<Image>, u32, u32), Color>);

/// The color to change the game screen to when paused.
const GREY_FILTER: Color = Color::LinearRgba(LinearRgba {
    red: 1.,
    green: 1.,
    blue: 1.,
    alpha: 1.0,
});

/// Adds grey filter to the game when paused.
fn grey_out_game(
    tile_textures: Query<&mut TilemapTexture>,
    mut images: ResMut<Assets<Image>>,
    mut orginal_colors: ResMut<OriginalTextureColors>,
) {
    for texture in tile_textures {
        let image_handles = texture.image_handles();
        for handle in image_handles {
            let image = images.get_mut(handle).unwrap();
            for x in 0..image.width() {
                for y in 0..image.height() {
                    if let Ok(original_color) = image.get_color_at(x, y) {
                        orginal_colors
                            .0
                            .insert((handle.clone_weak(), x, y), original_color);
                        let new_color = original_color.mix(&GREY_FILTER, 0.3);
                        image.set_color_at(x, y, new_color).unwrap_or_else(|e| {
                            error!("Unable to grey out texture: {}", e);
                            return;
                        });
                    }
                }
            }
        }
    }
}

/// Removes grey filter when game is resumed.
fn undo_greyed_out_game(
    mut images: ResMut<Assets<Image>>,
    orginal_colors: Res<OriginalTextureColors>,
) {
    for ((handle, x, y), color) in &orginal_colors.0 {
        if let Some(image) = images.get_mut(handle) {
            image.set_color_at(*x, *y, *color).unwrap_or_else(|e| {
                error!("Unable to restore original color: {}", e);
                return;
            })
        }
    }
}

/// Triggers `MapSpawnedEvent` when a tiled map has been loaded.
fn map_load_observer(
    _: Trigger<TiledMapCreated>,
    mut cmd: Commands,
    mut game_state: ResMut<NextState<GameState>>,
) {
    info!("{} map loaded!", MAP_NAME);
    cmd.trigger(MapLoadedEvent);
    game_state.set(GameState::Running);
}

/// Resumes the game when `ResumeGameEvent` is triggered.
fn resume_game_observer(_: Trigger<ResumeGameEvent>, mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Running);
    info!("Game resumed!");
}

/// Switches to the main menu screen.
fn switch_to_main_screen_observer(
    _: Trigger<ReturnToMainMenuEvent>,
    mut screen: ResMut<NextState<Screen>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    game_state.set(GameState::Exit);
    screen.set(Screen::Main);
}

mod menu {
    use bevy::{prelude::*, window::PrimaryWindow};
    use bevy_inspector_egui::{
        bevy_egui::{EguiContextPass, EguiContexts},
        egui::{self},
    };

    use crate::{
        screens::GameState,
        utils::{self, ResumeGameEvent, ReturnToMainMenuEvent},
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
        }
    }

    fn display_menu(
        mut cmd: Commands,
        mut egui_ctxs: EguiContexts,
        mut menu_state: ResMut<NextState<PauseMenuState>>,
        window: Single<&Window, With<PrimaryWindow>>,
    ) {
        if let Some(ctx) = egui_ctxs.try_ctx_mut() {
            menu_state.set(PauseMenuState::Main);

            let (window_size, window_position) = {
                let window_size = window.size();
                let size = (window_size.x * 0.5, window_size.y * 0.5);
                let position = (
                    (window_size.x / 2.) - (size.0 / 2.),
                    (window_size.y / 2.) - (size.1 / 2.),
                );
                (size, position)
            };

            egui::Window::new("Paused")
                .fixed_size(window_size)
                .current_pos(window_position)
                .movable(false)
                .resizable(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.set_width(ui.available_width());
                    ui.set_height(ui.available_height());

                    ui.vertical_centered(|ui| {
                        if menu_button(ui, "Resume").clicked() {
                            menu_state.set(PauseMenuState::Disabled);
                            cmd.trigger(ResumeGameEvent);
                        }

                        if menu_button(ui, "Main Menu").clicked() {
                            menu_state.set(PauseMenuState::Disabled);
                            cmd.trigger(ReturnToMainMenuEvent);
                        }
                    })
                });
        }
    }

    /// Creates a menu button.
    fn menu_button(ui: &mut egui::Ui, label: &str) -> egui::Response {
        utils::sized_button(ui, label, ui.available_width() * 0.7, 50.0)
    }
}
