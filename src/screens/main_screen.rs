use bevy::{input::common_conditions::input_just_pressed, prelude::*, ui::widget};

use crate::screens::Screen;

/// Bundles the systems of the `Main` screen.
pub(crate) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Main), setup);
    app.add_systems(
        Update,
        enter_gameplay_screen.run_if(input_just_pressed(KeyCode::Enter)),
    );
}

/// Setups up the camera and background mesh.
fn setup(mut cmd: Commands) {
    cmd.spawn((
        StateScoped(Screen::Main),
        widget::Text::new("Terra Firma"),
        children![(TextColor(Color::srgb(0.9, 0.9, 0.9)), TextShadow::default())],
    ));
}

/// Switches to the gameplay screen.
fn enter_gameplay_screen(mut next_state: ResMut<NextState<Screen>>) {
    next_state.set(Screen::Gameplay);
}
