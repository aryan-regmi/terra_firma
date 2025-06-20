use bevy::prelude::*;
use bevy_ecs_tiled::TiledMapPlugin;
use terra_firma::screens::{self};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    watch_for_changes_override: Some(true),
                    ..default()
                }),
            TiledMapPlugin::default(),
            screens::plugin,
        ))
        .add_systems(Startup, spawn_camera)
        .run();
}

/// Spawns the main camera.
fn spawn_camera(mut cmd: Commands) {
    cmd.spawn(Camera2d);
}
