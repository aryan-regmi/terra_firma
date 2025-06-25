use bevy::prelude::*;

pub(crate) mod gameplay_screen;
pub(crate) mod main_screen;

pub(crate) use gameplay_screen::*;
pub(crate) use main_screen::*;

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
#[states(scoped_entities)]
pub(crate) enum Screen {
    #[default]
    Main,
    Gameplay,
}

pub(crate) struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<Screen>();
        app.add_plugins((MainScreenPlugin, GameplayScreenPlugin));
    }
}
