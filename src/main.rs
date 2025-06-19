use bevy::prelude::*;
use terra_firma::game;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, game::GamePlugin))
        .run();
}
