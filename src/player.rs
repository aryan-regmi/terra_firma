use avian2d::prelude::*;
use bevy::{
    input::common_conditions::{input_just_pressed, input_just_released, input_pressed},
    prelude::*,
};

use crate::{
    animation::{self, AnimationConfig},
    helper::{self, CurrentMap, MapBounds},
    screens::Screen,
};

/// Determines the layer the player is drawn on.
const PLAYER_Z_IDX: f32 = 100.0;

/// Player movement speed factor.
const PLAYER_SPEED: f32 = 200.0;

/// How quickly should the camera snap to the desired location.
const CAMERA_DECAY_RATE: f32 = 2.;

/// Player sprite scale factor.
const PLAYER_SCALE: f32 = 2.0;

/// Marker component for the player.
#[derive(Component)]
pub(crate) struct Player;

/// Add the player systems to the app.
pub(crate) fn add_systems(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), setup);
    app.add_systems(
        Update,
        (move_player, move_camera).run_if(in_state(Screen::Gameplay)),
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

/// Updates the player's position.
fn move_player(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_position: Single<&mut Transform, With<Player>>,
    bounds: Res<MapBounds>,
) {
    let bounds = &bounds.0;

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
    bounds: Res<MapBounds>,
) {
    let bounds = &bounds.0;

    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    // Applies a smooth effect to camera movement using stable interpolation
    // between the camera position and the player position on the x and y axes.
    camera
        .translation
        .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
    camera.translation = camera.translation.clamp(
        Vec3::new(bounds.left, bounds.bottom, camera.translation.z),
        Vec3::new(bounds.right, bounds.top, camera.translation.z),
    );
}
