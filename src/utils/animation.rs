use bevy::prelude::*;

#[derive(Component, Default)]
/// Keeps track of an animation's configuration.
pub(crate) struct AnimationConfig {
    pub(crate) start_idx: usize,
    pub(crate) stop_idx: usize,
    pub(crate) timer: Timer,
}

/// Animates all sprites that have an animation timer..
pub(crate) fn animate_sprite<Filter: Component>(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut Sprite), With<Filter>>,
) {
    for (mut config, mut sprite) in &mut query {
        config.timer.tick(time.delta());

        if config.timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == config.stop_idx {
                    config.start_idx
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}
