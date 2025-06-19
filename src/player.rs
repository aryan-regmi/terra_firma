use bevy::prelude::*;

const PLAYER_Z_IDX: f32 = 1.0;
const PLAYER_SPEED: f32 = 200.0;

/// Marker component for the player.
#[derive(Component)]
pub struct Player;

#[derive(Debug, Component)]
pub enum SpriteType {
    First,
    Second,
}

pub fn setup(mut cmd: Commands, asset_server: Res<AssetServer>) {
    // Spawn character
    cmd.spawn((
        Player,
        SpriteType::First,
        Sprite::from_image(asset_server.load("tileset/moving-char1.png")),
        Transform::from_xyz(0., 0., PLAYER_Z_IDX).with_scale(Vec3::splat(1.2)),
    ));
}

// TODO: Add animations!
//  - Change sprite when the character is moved!

pub fn update(
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
