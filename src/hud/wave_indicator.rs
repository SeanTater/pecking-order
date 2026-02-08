use bevy::prelude::*;

use crate::game::waves::WaveManager;
use crate::states::GameState;

#[derive(Component)]
pub struct WaveText;

pub fn spawn_wave_indicator(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(16.0),
            left: Val::Px(0.0),
            right: Val::Px(0.0),
            justify_content: JustifyContent::Center,
            ..default()
        },
        DespawnOnExit(GameState::Playing),
    )).with_children(|parent| {
        parent.spawn((
            Text::new("Wave 1/5"),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            WaveText,
        ));
    });
}

pub fn update_wave_indicator(
    manager: Option<Res<WaveManager>>,
    mut query: Query<&mut Text, With<WaveText>>,
) {
    let Some(manager) = manager else { return };
    let Ok(mut text) = query.single_mut() else { return };

    if manager.all_complete() {
        **text = "All waves cleared!".into();
    } else {
        **text = format!("Wave {}/{}", manager.display_current(), manager.total());
    }
}
