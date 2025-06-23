use crate::{
    helper,
    player::{self, Player},
    screens::Screen,
};
// use avian2d::prelude::*;
use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_ecs_tilemap::prelude::*;

/// Map scale factor.
const MAP_SCALE: f32 = 2.0;

/// Marker component for the background mesh.
#[derive(Component)]
struct BackgroundMesh;

/// Bundles the systems of the `Gameplay` screen.
pub(crate) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), setup);
    app.add_systems(OnExit(Screen::Gameplay), despawn_player);
    app.add_plugins((crate::tiled::TiledMapPlugin, TilemapPlugin));
    app.add_systems(
        Update,
        enter_main_screen.run_if(input_just_pressed(KeyCode::Escape)),
    );
    player::add_systems(app);
}

/// Setups up the camera and spawns the map.
fn setup(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let mut projection = OrthographicProjection::default_2d();
    projection.scaling_mode = bevy::render::camera::ScalingMode::WindowSize;
    let map_handle = crate::tiled::TiledMapHandle(asset_server.load("maps/map_00/main.tmx"));
    cmd.spawn((
        StateScoped(Screen::Gameplay),
        crate::tiled::TiledMapBundle {
            name: helper::Name("Main".into()),
            tiled_map: map_handle,
            transform: Transform::default().with_scale(Vec3::splat(MAP_SCALE)),
            ..default()
        },
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
