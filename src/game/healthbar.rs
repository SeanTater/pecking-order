//! World-space health bars rendered as child sprites above characters.

use bevy::prelude::*;

use super::combat::Health;
use super::player::Player;

/// Marker on the foreground (colored) bar sprite.
#[derive(Component)]
pub struct HealthBarFg {
    /// The entity whose Health we track.
    pub owner: Entity,
    pub max_width: f32,
}

/// Marker on the background (dark) bar sprite.
#[derive(Component)]
pub struct HealthBarBg;

const PLAYER_BAR_WIDTH: f32 = 32.0;
const ENEMY_BAR_WIDTH: f32 = 20.0;
const BAR_HEIGHT: f32 = 3.0;
const BAR_Y_OFFSET: f32 = 24.0;

/// Attach health bars to newly spawned entities with Health.
pub fn spawn_healthbars(
    mut commands: Commands,
    new_health: Query<(Entity, &Health, Option<&Player>), Added<Health>>,
) {
    for (entity, _health, is_player) in &new_health {
        let max_width = if is_player.is_some() {
            PLAYER_BAR_WIDTH
        } else {
            ENEMY_BAR_WIDTH
        };

        let fg_color = if is_player.is_some() {
            Color::srgb(0.2, 0.8, 0.2)
        } else {
            Color::srgb(0.8, 0.2, 0.2)
        };

        // Background bar (dark)
        let bg = commands
            .spawn((
                Sprite {
                    color: Color::srgba(0.0, 0.0, 0.0, 0.6),
                    custom_size: Some(Vec2::new(max_width + 2.0, BAR_HEIGHT + 2.0)),
                    ..default()
                },
                Transform::from_translation(Vec3::new(0.0, BAR_Y_OFFSET, 5.0)),
                HealthBarBg,
            ))
            .id();

        // Foreground bar (colored)
        let fg = commands
            .spawn((
                Sprite {
                    color: fg_color,
                    custom_size: Some(Vec2::new(max_width, BAR_HEIGHT)),
                    ..default()
                },
                Transform::from_translation(Vec3::new(0.0, BAR_Y_OFFSET, 6.0)),
                HealthBarFg {
                    owner: entity,
                    max_width,
                },
            ))
            .id();

        commands.entity(entity).add_children(&[bg, fg]);
    }
}

/// Update foreground bar width and color based on current health.
pub fn update_healthbars(
    health_q: Query<&Health>,
    mut bar_q: Query<(&HealthBarFg, &mut Sprite)>,
) {
    for (bar, mut sprite) in &mut bar_q {
        let Ok(health) = health_q.get(bar.owner) else {
            continue;
        };
        let pct = (health.current as f32) / (health.max as f32).max(1.0);
        let pct = pct.clamp(0.0, 1.0);

        // Scale width
        if let Some(ref mut size) = sprite.custom_size {
            size.x = bar.max_width * pct;
        }

        // Color: green → yellow → red
        sprite.color = health_color(pct);
    }
}

/// Pure function: map health percentage to a color (green→yellow→red).
pub fn health_color(pct: f32) -> Color {
    if pct > 0.5 {
        // green to yellow
        let t = (pct - 0.5) * 2.0;
        Color::srgb(1.0 - t, 0.8, 0.2 * (1.0 - t))
    } else {
        // yellow to red
        let t = pct * 2.0;
        Color::srgb(0.9, t * 0.8, 0.0)
    }
}
