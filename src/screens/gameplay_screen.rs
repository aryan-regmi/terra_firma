use bevy::{input::common_conditions::input_pressed, prelude::*};
use bevy_ecs_tiled::{map::TiledMapHandle, prelude::*};

use crate::{screens::Screen, utils};

pub struct GameplayScreenPlugin;

impl Plugin for GameplayScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Gameplay), load_main_map);
        app.add_systems(
            Update,
            switch_to_main_screen
                .run_if(input_pressed(KeyCode::Escape).and(in_state(Screen::Gameplay))),
        );
    }
}

// FIXME: Handle map loading (show different screen while loading)!
fn load_main_map(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    mut maps: ResMut<utils::Tilemaps>,
) {
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

/// Switches to the gameplay screen.
fn switch_to_main_screen(mut next_state: ResMut<NextState<Screen>>) {
    next_state.set(Screen::Main);
}
