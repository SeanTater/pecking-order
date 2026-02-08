use bevy::prelude::*;

use crate::game::combat::Health;
use crate::game::player::Player;
use crate::states::GameState;

#[derive(Component)]
pub struct HeartIcon {
    pub index: i32,
}

#[derive(Component)]
pub struct HeartsContainer;

pub fn spawn_hearts(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(16.0),
                top: Val::Px(16.0),
                column_gap: Val::Px(4.0),
                ..default()
            },
            HeartsContainer,
            DespawnOnExit(GameState::Playing),
        ))
        .with_children(|parent| {
            for i in 0..10 {
                parent.spawn((
                    Node {
                        width: Val::Px(20.0),
                        height: Val::Px(20.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.8, 0.1, 0.1)),
                    HeartIcon { index: i },
                ));
            }
        });
}

pub fn update_hearts(
    player_q: Query<&Health, With<Player>>,
    mut hearts: Query<(&HeartIcon, &mut BackgroundColor)>,
) {
    let Ok(health) = player_q.single() else {
        return;
    };

    for (heart, mut bg) in &mut hearts {
        if heart.index < health.current {
            *bg = BackgroundColor(Color::srgb(0.8, 0.1, 0.1));
        } else {
            *bg = BackgroundColor(Color::srgb(0.3, 0.3, 0.3));
        }
    }
}
