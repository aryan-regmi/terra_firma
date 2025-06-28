use avian2d::PhysicsPlugins;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::{screens, utils};

/// Manages resources and systems for the entire game.
#[derive(Debug, Default)]
pub struct GamePlugin {
    /// Determines whether to show the `egui` inspector or not.
    pub inspector: bool,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<utils::Tilemaps>()
            .register_type::<utils::Tilemaps>();
        app.register_type::<utils::ChunkSize>();
        if self.inspector {
            app.add_plugins(WorldInspectorPlugin::new());
        }
        app.add_plugins((
            TiledMapPlugin::default(),
            screens::ScreenPlugin,
            TiledPhysicsPlugin::<TiledPhysicsAvianBackend>::default(),
            PhysicsPlugins::default().with_length_unit(500.0),
        ));
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut cmd: Commands) {
    // Spawn a Bevy 2D camera
    cmd.spawn(Camera2d);
}
