//! Pause menu: toggle pause, overlay UI with Resume/Quit buttons.

use bevy::prelude::*;

use crate::states::GameState;

/// Toggle between Playing and Paused on Escape.
pub(super) fn toggle_pause(
    keyboard: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match state.get() {
            GameState::Playing => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::Playing),
            _ => {}
        }
    }
}

/// Marker for the Resume button.
#[derive(Component)]
pub(super) struct ResumeButton;

/// Marker for the Quit button.
#[derive(Component)]
pub(super) struct QuitButton;

pub(super) fn spawn_pause_overlay(mut commands: Commands) {
    // Semi-transparent full-screen overlay
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
            GlobalZIndex(100),
            DespawnOnExit(GameState::Paused),
        ))
        .with_children(|parent| {
            // "Paused" title
            parent.spawn((
                Text::new("PAUSED"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Resume button
            parent
                .spawn((
                    Node {
                        padding: UiRect::axes(Val::Px(24.0), Val::Px(12.0)),
                        ..default()
                    },
                    Button,
                    BackgroundColor(Color::srgb(0.25, 0.55, 0.25)),
                    ResumeButton,
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new("Resume"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            // Quit button
            parent
                .spawn((
                    Node {
                        padding: UiRect::axes(Val::Px(24.0), Val::Px(12.0)),
                        ..default()
                    },
                    Button,
                    BackgroundColor(Color::srgb(0.6, 0.2, 0.2)),
                    QuitButton,
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new("Quit"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

pub(super) fn pause_menu_input(
    resume_q: Query<&Interaction, (Changed<Interaction>, With<ResumeButton>)>,
    quit_q: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &resume_q {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Playing);
        }
    }
    for interaction in &quit_q {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::MainMenu);
        }
    }
}
