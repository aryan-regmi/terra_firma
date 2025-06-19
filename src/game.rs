use bevy::{
    image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    math::Affine2,
    prelude::*,
};

use crate::player;

/// Marker component for the background mesh.
#[derive(Component)]
struct BackgroundMesh;

#[derive(Debug)]
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::setup);
        app.add_plugins(player::PlayerPlugin);
    }
}

impl GamePlugin {
    const BACKGROUND_Z_IDX: f32 = 0.;

    fn setup(
        mut cmd: Commands,
        asset_server: Res<AssetServer>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        window: Single<&Window>,
    ) {
        // Spawn camera
        cmd.spawn(Camera2d);

        // Spawn background mesh
        {
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

            let mesh_size = window.physical_size();
            let grid_num_cells = Vec2::new(20., 20.);
            cmd.spawn((
                BackgroundMesh,
                Mesh2d(meshes.add(Rectangle::new(mesh_size.x as f32, mesh_size.y as f32))),
                MeshMaterial2d(materials.add(ColorMaterial {
                    texture: Some(background_tile),
                    uv_transform: Affine2::from_scale(grid_num_cells),
                    ..default()
                })),
                Transform::from_xyz(0., 0., Self::BACKGROUND_Z_IDX),
            ));
        }
    }
}
