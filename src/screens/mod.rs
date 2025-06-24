use bevy::prelude::*;

pub mod gameplay_screen;
pub mod main_screen;

pub use gameplay_screen::*;
pub use main_screen::*;

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
#[states(scoped_entities)]
pub(crate) enum Screen {
    #[default]
    Main,
    Gameplay,
}

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<Screen>();
        app.add_plugins((MainScreenPlugin, GameplayScreenPlugin));
    }
}
