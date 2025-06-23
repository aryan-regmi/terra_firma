pub mod animation;
pub mod button;
pub mod player;
pub mod screens;
pub mod tiled;

pub mod helper {
    use bevy::prelude::*;

    /// Component for a name.
    #[derive(Component, PartialEq, Eq, Default, Debug)]
    pub struct Name(pub(crate) String);

    #[derive(Resource)]
    pub struct CurrentMap(pub Name);

    #[allow(unused)]
    #[derive(Debug, Default)]
    pub struct Bounds {
        pub left: f32,
        pub right: f32,
        pub top: f32,
        pub bottom: f32,
    }
}

// TODO: Add egui inspector plugin!
//
// TODO: Add loader to load all assets in the beginning!
//  - Handle when asset isn't fully loaded
//
// TODO: Add animation using bevy_ecs_tilemap
//
// TODO: Add all `scale` constants as resources
//
// TODO: Replace external assests with custom ones!
//
// TODO: Make camera follow player
//
// TODO: Add `Zone { name: string }` property to tiles -> Adds a Zone component
//  - Can query for this component to change maps when a new zone is entered
//  - Setup maps the same way as screens
//
// TODO: Add `Zone { name: string }` property to tiles -> Adds a Zone component
//  - Can query for this component to change maps when a new zone is entered
//  - Setup maps the same way as screens
//
//
// TODO: Add `Animated { sprite_sheet, start_idx, end_idx, duration }` property to tiles -> flags
// for animation
//  - Add to the tiled plugin, so map tiles can be individually animated
