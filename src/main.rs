use bevy::prelude::*;
use terra_firma::screens::{self};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            screens::plugin,
        ))
        .add_systems(Startup, spawn_camera)
        .run();
}

fn spawn_camera(mut cmd: Commands) {
    cmd.spawn(Camera2d);
}
