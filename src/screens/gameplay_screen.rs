use crate::{
    player::{self, Player},
    screens::Screen,
};
use avian2d::prelude::*;
use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_ecs_tiled::prelude::*;

const MAP_SCALE: f32 = 2.0;

/// Marker component for the background mesh.
#[derive(Component)]
struct BackgroundMesh;

// Declare a component and make it "reflectable"
#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component, Default)]
struct TileInfo(bool);
// pub struct TileInfo {
//     is_collider: bool,
// }

/// Bundles the systems of the `Gameplay` screen.
pub(crate) fn plugin(app: &mut App) {
    app.register_type::<TileInfo>();
    app.add_systems(OnEnter(Screen::Gameplay), (setup, add_colliders).chain());
    app.add_systems(OnExit(Screen::Gameplay), despawn_player);
    app.add_plugins((
        TiledMapPlugin::default(),
        TiledPhysicsPlugin::<TiledPhysicsAvianBackend>::default(),
    ));
    app.add_systems(
        Update,
        enter_main_screen.run_if(input_just_pressed(KeyCode::Escape)),
    );
    player::add_systems(app);
}

/// Setups up the camera and spawns the map.
fn setup(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let map_handle: Handle<TiledMap> = asset_server.load("maps/map_00/main.tmx");
    cmd.spawn((
        StateScoped(Screen::Gameplay),
        TiledMapHandle(map_handle),
        TilemapAnchor::Center,
        TiledPhysicsSettings::<TiledPhysicsAvianBackend> {
            tiles_layer_filter: TiledName::All,
            objects_filter: TiledName::All,
            objects_layer_filter: TiledName::All,
            ..default()
        },
        Transform::default().with_scale(Vec3::splat(MAP_SCALE)),
    ));
}

fn add_colliders(mut cmd: Commands, asset_server: Res<AssetServer>, colliders: Query<&Collider>) {
    for (i, pos) in colliders.iter().enumerate() {
        dbg!(i);
        // if let Some(TileInfo(true)) = info {
        // cmd.spawn((
        //     Sprite {
        //         image: asset_server.load("tileset/character-sprite-sheet.png"),
        //         ..default()
        //     },
        //     RigidBody::Static,
        //     Collider::rectangle(32.0, 32.0),
        //     Transform::from_xyz(pos.x as f32, pos.y as f32, 3.0),
        // ));
        // }
    }
}

/// Switches to the main screen.
fn enter_main_screen(mut next_state: ResMut<NextState<Screen>>) {
    next_state.set(Screen::Main);
}

/// Removes the player from the game.
fn despawn_player(mut cmd: Commands, player: Single<Entity, With<Player>>) {
    cmd.entity(*player).despawn();
}
