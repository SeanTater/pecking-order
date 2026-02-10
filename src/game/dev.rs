//! Dev keybinds (debug builds only).

use bevy::prelude::*;

use super::combat::Health;
use super::enemy::Enemy;
use super::player::Player;
use super::waves::WaveManager;

pub fn dev_keybinds(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_q: Query<&mut Health, With<Player>>,
    enemies: Query<Entity, With<Enemy>>,
    mut commands: Commands,
    wave_mgr: Option<Res<WaveManager>>,
) {
    // G — god mode: refill health
    if keyboard.pressed(KeyCode::KeyG) {
        if let Ok(mut health) = player_q.single_mut() {
            health.current = health.max;
        }
    }

    // N — skip wave: despawn all enemies
    if keyboard.just_pressed(KeyCode::KeyN) {
        for entity in &enemies {
            commands.entity(entity).despawn();
        }
    }

    // F1 — log debug info
    if keyboard.just_pressed(KeyCode::F1) {
        if let Some(wm) = &wave_mgr {
            info!("Wave {}/{}", wm.display_current(), wm.total());
        }
        if let Ok(health) = player_q.single() {
            info!("Player HP: {}/{}", health.current, health.max);
        }
    }
}
