use bevy::prelude::*;

use crate::{screens::GameState, utils};

/// Manages resources and systems for the player.
#[derive(Debug, Default)]
pub(crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_player_observer);
        app.add_systems(
            Update,
            utils::animate_sprite::<PlayerMarker>.run_if(in_state(GameState::Running)),
        );
    }
}

#[derive(Default, Component)]
/// Marker component for the player.
pub(crate) struct PlayerMarker;

/// Size of the player tile.
const PLAYER_TILE_SIZE: (u32, u32) = (32, 32);

/// Scale of the player texture.
const PLAYER_SCALE: f32 = 2.0;

/// Number of columns in the player idle sprite sheet.
const PLAYER_IDLE_SPRITE_NCOLS: u32 = 3;

/// The time (in seconds) to run the animation for.
const PLAYER_IDLE_ANIMATION_TIME: f32 = 0.25;

fn spawn_player_observer(
    _: Trigger<utils::MapLoadedEvent>,
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("player/player-idle-sheet.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(PLAYER_TILE_SIZE.0, PLAYER_TILE_SIZE.1),
        PLAYER_IDLE_SPRITE_NCOLS,
        1,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_config = utils::AnimationConfig {
        start_idx: 0,
        stop_idx: PLAYER_IDLE_SPRITE_NCOLS as usize - 1,
        timer: Timer::from_seconds(PLAYER_IDLE_ANIMATION_TIME, TimerMode::Repeating),
    };

    cmd.spawn((
        PlayerMarker,
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_config.start_idx,
            },
        ),
        Transform::default().with_scale(Vec3::splat(PLAYER_SCALE)),
        animation_config,
    ));

    info!("Player spawned!");
}
