use crate::{
    player::{self, Player},
    screens::Screen,
};
use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_ecs_tiled::prelude::*;

const MAP_SCALE: f32 = 2.0;

/// Bundles the systems of the `Gameplay` screen.
pub(crate) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), setup);
    app.add_systems(OnExit(Screen::Gameplay), despawn_player);
    app.add_systems(
        Update,
        enter_main_screen.run_if(input_just_pressed(KeyCode::Escape)),
    );
    player::add_systems(app);
}

/// Marker component for the background mesh.
#[derive(Component)]
struct BackgroundMesh;

/// Setups up the camera and background mesh.
fn setup(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let map_handle: Handle<TiledMap> = asset_server.load("maps/map_01.tmx");
    cmd.spawn((
        StateScoped(Screen::Gameplay),
        TiledMapHandle(map_handle),
        TilemapAnchor::Center,
        Transform::default().with_scale(Vec3::splat(MAP_SCALE)),
    ));
}

/// Switches to the main screen.
fn enter_main_screen(mut next_state: ResMut<NextState<Screen>>) {
    next_state.set(Screen::Main);
}

/// Removes the player from the game.
fn despawn_player(mut cmd: Commands, player: Single<Entity, With<Player>>) {
    cmd.entity(*player).despawn();
}
