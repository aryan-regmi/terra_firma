use bevy::prelude::*;

pub mod gameplay_screen;
pub mod main_screen;

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
#[states(scoped_entities)]
pub enum Screen {
    #[default]
    Main,
    Gameplay,
}

pub fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.add_plugins((main_screen::plugin, gameplay_screen::plugin));
}
