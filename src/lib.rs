pub mod animation;
pub mod button;
pub mod player;
pub mod screens;

mod tiled;

pub mod helper {
    pub use crate::tiled::*;
}

// TODO: Add egui inspector plugin!
//
// TODO: Add loader to load all assets in the beginning!
//  - Handle when asset isn't fully loaded
//
// TODO: Add animation using bevy_ecs_tilemap
//
// TODO: Add all `scale` constants as resources
