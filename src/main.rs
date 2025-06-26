use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use terra_firma::GamePlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    watch_for_changes_override: Some(true),
                    ..default()
                }),
        )
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(GamePlugin { inspector: true })
        .run();
}
