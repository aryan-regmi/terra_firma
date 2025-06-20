use bevy::prelude::*;
use std::time::Duration;

// Starts the animation.
pub fn start_animation<S: Component>(mut animation: Single<&mut AnimationConfig, With<S>>) {
    animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps, TimerMode::Repeating);
}

// Restarts the animation.
pub fn stop_animation<S: Component>(mut animation: Single<&mut AnimationConfig, With<S>>) {
    animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps, TimerMode::Once);
}

#[derive(Component)]
pub struct AnimationConfig {
    pub(crate) first_sprite_index: usize,
    pub(crate) last_sprite_index: usize,
    pub(crate) fps: f32,
    pub(crate) frame_timer: Timer,
}

impl AnimationConfig {
    /// Creates a new config given the first and last sprite index in the atlas, and the fps of the
    /// animation.
    pub fn new(first: usize, last: usize, fps: f32, timer_mode: TimerMode) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps, timer_mode),
        }
    }

    fn timer_from_fps(fps: f32, timer_mode: TimerMode) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / fps), timer_mode)
    }
}

// This system loops through all the sprites in the `TextureAtlas`, from  `first_sprite_index` to
// `last_sprite_index` (both defined in `AnimationConfig`).
pub fn execute_animations(
    time: Res<Time>,
    mut animation_info: Query<(&mut AnimationConfig, &mut Sprite)>,
) {
    // let (ref mut config, ref mut sprite) = &mut *animation_info;
    for (mut config, mut sprite) in &mut animation_info {
        // We track how long the current sprite has been displayed for
        config.frame_timer.tick(time.delta());

        // If it has been displayed for the user-defined amount of time (fps)...
        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index == config.last_sprite_index {
                    atlas.index = config.first_sprite_index;
                } else {
                    atlas.index += 1;
                    config.frame_timer =
                        AnimationConfig::timer_from_fps(config.fps, config.frame_timer.mode());
                }
            }
        }
    }
}
