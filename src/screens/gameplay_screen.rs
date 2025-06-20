use crate::{
    player::{self, Player},
    screens::Screen,
};
use bevy::{
    image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    input::common_conditions::input_just_pressed,
    math::Affine2,
    prelude::*,
};

/// The Z-Index of the background.
const BACKGROUND_Z_IDX: f32 = 0.;

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
fn setup(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    window: Single<&Window>,
) {
    cmd.spawn((
        StateScoped(Screen::Gameplay),
        background_mesh(asset_server, meshes, materials, window),
    ));
}

/// Spawns the background mesh.
fn background_mesh(
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Single<&Window>,
) -> impl Bundle {
    let mesh_size = window.physical_size();
    let grid_num_cells = Vec2::new(20., 20.);
    let background_tile = asset_server.load_with_settings("tileset/grass-01.png", |s| {
        *s = ImageLoaderSettings {
            sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                address_mode_u: ImageAddressMode::Repeat,
                address_mode_v: ImageAddressMode::Repeat,
                ..Default::default()
            }),
            ..Default::default()
        }
    });

    (
        BackgroundMesh,
        Mesh2d(meshes.add(Rectangle::new(mesh_size.x as f32, mesh_size.y as f32))),
        MeshMaterial2d(materials.add(ColorMaterial {
            texture: Some(background_tile),
            uv_transform: Affine2::from_scale(grid_num_cells),
            ..default()
        })),
        Transform::from_xyz(0., 0., BACKGROUND_Z_IDX),
    )
}

/// Switches to the main screen.
fn enter_main_screen(mut next_state: ResMut<NextState<Screen>>) {
    next_state.set(Screen::Main);
}

/// Removes the player from the game.
fn despawn_player(mut cmd: Commands, player: Single<Entity, With<Player>>) {
    cmd.entity(*player).despawn();
}
