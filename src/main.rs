mod loading;
mod menu;

use bevy::prelude::*;
use pecking_order::states::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pecking Order".into(),
                resolution: (1280u32, 720u32).into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_plugins((
            loading::LoadingPlugin,
            menu::MenuPlugin,
            pecking_order::game::GamePlugin,
            pecking_order::hud::HudPlugin,
        ))
        .run();
}
