use bevy::{
    input::common_conditions::{input_just_pressed, input_just_released, input_pressed},
    prelude::*,
};

use crate::{
    animation::{self, AnimationConfig},
    screens::Screen,
};

const PLAYER_Z_IDX: f32 = 1.0;
const PLAYER_SPEED: f32 = 200.0;
const PLAYER_SCALE: f32 = 2.0;

/// Marker component for the player.
#[derive(Component)]
pub(crate) struct Player;

/// Add the player systems to the app.
pub(crate) fn add_systems(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), setup);
    app.add_systems(Update, update_position.run_if(in_state(Screen::Gameplay)));
    update_animations(app);
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
        Transform::from_xyz(0., 0., PLAYER_Z_IDX).with_scale(Vec3::splat(PLAYER_SCALE)),
    ));
}

/// Updates the player's position.
fn update_position(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_position: Single<&mut Transform, With<Player>>,
    window: Single<&Window>,
) {
    let (left_bound, right_bound, top_bound, bottom_bound) = {
        let window_size = window.resolution.size();
        (
            -window_size.x / 2.,
            window_size.x / 2.,
            -window_size.y / 2.,
            window_size.y / 2.,
        )
    };

    // Handle input
    let mut x_direction = 0.0;
    let mut y_direction = 0.0;
    {
        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            y_direction += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            x_direction -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            y_direction -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            x_direction += 1.0;
        }
    }

    // Update player transform
    let updated_transform_x =
        player_position.translation.x + x_direction * PLAYER_SPEED * time.delta_secs();
    let updated_transform_y =
        player_position.translation.y + y_direction * PLAYER_SPEED * time.delta_secs();
    player_position.translation.x = updated_transform_x.clamp(left_bound, right_bound);
    player_position.translation.y = updated_transform_y.clamp(top_bound, bottom_bound);
}

/// Updates animations for the player.
fn update_animations(app: &mut App) {
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
