use avian2d::prelude::*;
use bevy::prelude::*;
use terra_firma::screens::{self};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            PhysicsPlugins::default().with_length_unit(100.0),
            screens::plugin,
        ))
        .add_systems(Startup, spawn_camera)
        .run();
}

/// Spawns the main camera.
fn spawn_camera(mut cmd: Commands) {
    cmd.spawn(Camera2d);
}
