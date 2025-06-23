use avian2d::prelude::*;
use bevy::{
    input::common_conditions::{input_just_pressed, input_just_released, input_pressed},
    prelude::*,
};

use crate::{
    animation::{self, AnimationConfig},
    screens::Screen,
};

/// Determines the layer the player is drawn on.
const PLAYER_Z_IDX: f32 = 100.0;

/// Player movement speed factor.
const PLAYER_SPEED: f32 = 200.0;

/// Player sprite scale factor.
const PLAYER_SCALE: f32 = 2.0;

/// How quickly should the camera snap to the desired location.
const CAMERA_DECAY_RATE: f32 = 2.;

/// Marker component for the player.
#[derive(Component)]
pub(crate) struct Player;

#[derive(Resource)]
struct CurrentMap(crate::helper::Name);

#[allow(unused)]
#[derive(Debug, Default)]
struct Bounds {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

/// Add the player systems to the app.
pub(crate) fn add_systems(app: &mut App) {
    app.insert_resource(CurrentMap(crate::helper::Name("Main".into())));
    app.add_systems(OnEnter(Screen::Gameplay), setup);
    app.add_systems(
        Update,
        (move_player, move_camera_2).run_if(in_state(Screen::Gameplay)),
    );
    add_animation_systems(app);
}

/// Create and spawn the player.
fn setup(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Setup sprite
    let texture = asset_server.load("tileset/character-sprite-sheet.png");
    let num_rows = 1;
    let num_cols = 4;
    let sprite_size = UVec2::splat(32);
    let fps = 20.;
    let layout = TextureAtlasLayout::from_grid(sprite_size, num_cols, num_rows, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_config = AnimationConfig::new(0, 3, fps, TimerMode::Once);

    // Spawn character
    cmd.spawn((
        Player,
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation_config.first_sprite_index,
            }),
            ..default()
        },
        animation_config,
        RigidBody::Kinematic,
        Collider::rectangle(sprite_size.x as f32, sprite_size.y as f32),
        Transform::from_xyz(0., 0., PLAYER_Z_IDX).with_scale(Vec3::splat(PLAYER_SCALE)),
    ));
}

// FIXME: Do this in setup!
//
/// Claculates the bounds of the current map.
fn calculate_bounds(
    current_map: Res<CurrentMap>,
    maps: Res<Assets<crate::helper::TiledMap>>,
    window: Single<&Window>,
    tilemaps: Query<(&crate::helper::Name, &crate::helper::TiledMapHandle)>,
) -> Bounds {
    let mut bounds = Bounds::default();
    for (name, tilemap) in tilemaps {
        if *name == current_map.0 {
            let tiled_map = maps.get(&tilemap.0);
            if let Some(tiled_map) = tiled_map {
                let (map_width, map_height) =
                    (tiled_map.map.width as f32, tiled_map.map.height as f32);
                let (tile_width, tile_height) = (
                    tiled_map.map.tile_width as f32,
                    tiled_map.map.tile_height as f32,
                );
                let right_bound = map_width * tile_width;
                let top_bound = map_height * tile_height;
                let left_bound = -right_bound;
                let bottom_bound = -top_bound;
                bounds = Bounds {
                    left: left_bound,
                    right: right_bound,
                    top: top_bound,
                    bottom: bottom_bound,
                };
                break;
            } else {
                let window_size = window.size();
                bounds = Bounds {
                    left: -window_size.x,
                    right: window_size.x,
                    top: window_size.y,
                    bottom: -window_size.y,
                };
                break;
            }
        } else {
            let window_size = window.size();
            bounds = Bounds {
                left: -window_size.x,
                right: window_size.x,
                top: window_size.y,
                bottom: -window_size.y,
            };
            break;
        }
    }
    bounds
}

/// Updates the player's position.
fn move_player(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    current_map: Res<CurrentMap>,
    maps: Res<Assets<crate::helper::TiledMap>>,
    mut player_position: Single<&mut Transform, With<Player>>,
    window: Single<&Window>,
    tilemaps: Query<(&crate::helper::Name, &crate::helper::TiledMapHandle)>,
) {
    let bounds = calculate_bounds(current_map, maps, window, tilemaps);

    // Handle input
    let mut direction = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        if player_position.translation.y < bounds.top {
            direction.y += 1.0;
        }
    }
    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        if player_position.translation.x > bounds.left {
            direction.x -= 1.0;
        }
    }
    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        if player_position.translation.y > bounds.bottom {
            direction.y -= 1.0;
        }
    }
    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        if player_position.translation.x < bounds.right {
            direction.x += 1.0;
        }
    }

    // Progressively update the player's position over time. Normalize the
    // direction vector to prevent it from exceeding a magnitude of 1 when
    // moving diagonally.
    let move_delta = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs();
    let updated_translation = player_position.translation + move_delta.extend(0.);
    player_position.translation = updated_translation;
}

/// Adds the animation systems for the player.
fn add_animation_systems(app: &mut App) {
    // FIXME: Replace run_if's with keyboard_input handling in the function instead!
    //  - Fix animations etc by checking youtube video in `Bevy` playlist
    app.add_systems(
        Update,
        (
            animation::execute_animations,
            animation::start_animation::<Player>.run_if(
                input_just_pressed(KeyCode::KeyW)
                    .or(input_just_pressed(KeyCode::KeyA))
                    .or(input_just_pressed(KeyCode::KeyS))
                    .or(input_just_pressed(KeyCode::KeyD)),
            ),
            animation::stop_animation::<Player>.run_if(
                input_just_released(KeyCode::KeyW)
                    .xor(input_just_released(KeyCode::KeyA))
                    .xor(input_just_released(KeyCode::KeyS))
                    .xor(input_just_released(KeyCode::KeyD))
                    .and(not(input_pressed(KeyCode::KeyW)
                        .or(input_pressed(KeyCode::KeyA))
                        .or(input_pressed(KeyCode::KeyS))
                        .or(input_pressed(KeyCode::KeyD)))),
            ),
        )
            .run_if(in_state(Screen::Gameplay)),
    );
}

/// Makes the camera follow the player.
fn move_camera(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
    current_map: Res<CurrentMap>,
    maps: Res<Assets<crate::helper::TiledMap>>,
    tilemaps: Query<(&crate::helper::Name, &crate::helper::TiledMapHandle)>,
    window: Single<&Window>,
) {
    let Vec3 { x, y, .. } = player.translation;
    // let direction = Vec3::new(x, y, camera.translation.z);

    // if player_position.translation.y < bounds.top {
    //     direction.y += 1.0;
    // }

    let direction = Vec3::new(x, y, camera.translation.z);

    let bounds = {
        let mut bounds = calculate_bounds(current_map, maps, window, tilemaps);
        bounds.left = bounds.left + CAMERA_BOUNDS.left;
        bounds.right = bounds.right - CAMERA_BOUNDS.right;
        bounds.top = bounds.top - CAMERA_BOUNDS.top;
        bounds.bottom = bounds.bottom + CAMERA_BOUNDS.bottom;
        bounds
    };

    if player.translation.y > bounds.top {
        camera.translation.y = bounds.top;
    }
    if player.translation.x < bounds.left {}
    if player.translation.y > bounds.bottom {}
    if player.translation.x < bounds.right {}

    camera
        .translation
        .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
}

fn move_camera_2(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    current_map: Res<CurrentMap>,
    maps: Res<Assets<crate::helper::TiledMap>>,
    window: Single<&Window>,
    tilemaps: Query<(&crate::helper::Name, &crate::helper::TiledMapHandle)>,
) {
    let mut bounds = calculate_bounds(current_map, maps, window, tilemaps);
    bounds = Bounds {
        left: 0.2 * bounds.left,
        right: 0.2 * bounds.right,
        top: 0.2 * bounds.top,
        bottom: 0.2 * bounds.bottom,
    };

    // Handle input
    let mut direction = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        if camera.translation.y < bounds.top {
            direction.y += 1.0;
        }
    }
    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        if camera.translation.x > bounds.left {
            direction.x -= 1.0;
        }
    }
    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        if camera.translation.y > bounds.bottom {
            direction.y -= 1.0;
        }
    }
    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        if camera.translation.x < bounds.right {
            direction.x += 1.0;
        }
    }
    // info_once!("{:?}", bounds);
    dbg!(bounds);

    // Progressively update the player's position over time. Normalize the
    // direction vector to prevent it from exceeding a magnitude of 1 when
    // moving diagonally.
    let move_delta = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs();
    let updated_translation = camera.translation + move_delta.extend(0.);
    camera.translation = updated_translation;
    dbg!(camera.translation);
}
