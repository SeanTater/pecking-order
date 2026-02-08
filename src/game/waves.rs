//! Wave system: authored sequences of enemy spawns with breathing room between waves.

use bevy::prelude::*;
use std::f32::consts::TAU;

use crate::states::GameState;
use super::combat::Health;
use super::enemy::{Enemy, RushBehavior};

// ── Enemy types ──────────────────────────────────────────────────────

#[derive(Clone, Copy)]
pub enum EnemyType {
    Ant,
    GardenSnake,
}

impl EnemyType {
    pub fn health(&self) -> i32 {
        match self {
            EnemyType::Ant => 1,
            EnemyType::GardenSnake => 4,
        }
    }

    pub fn speed(&self) -> f32 {
        match self {
            EnemyType::Ant => 60.0,
            EnemyType::GardenSnake => 110.0,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            EnemyType::Ant => Color::srgb(0.15, 0.1, 0.05),
            EnemyType::GardenSnake => Color::srgb(0.2, 0.5, 0.15),
        }
    }

    pub fn size(&self) -> Vec2 {
        match self {
            EnemyType::Ant => Vec2::new(12.0, 12.0),
            EnemyType::GardenSnake => Vec2::new(20.0, 10.0),
        }
    }
}

// ── Wave data ────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct SpawnGroup {
    pub enemy_type: EnemyType,
    pub count: usize,
}

#[derive(Clone)]
pub struct Wave {
    pub groups: Vec<SpawnGroup>,
}

#[derive(Resource)]
pub struct WaveManager {
    pub waves: Vec<Wave>,
    pub current: usize,
    /// Brief pause between waves so the player can breathe.
    pub cooldown: Timer,
    pub spawned_current: bool,
}

impl WaveManager {
    pub fn total(&self) -> usize {
        self.waves.len()
    }

    pub fn display_current(&self) -> usize {
        // 1-indexed for HUD
        (self.current + 1).min(self.waves.len())
    }

    pub fn all_complete(&self) -> bool {
        self.current >= self.waves.len()
    }
}

// ── Systems ──────────────────────────────────────────────────────────

pub fn init_waves(mut commands: Commands) {
    commands.insert_resource(WaveManager {
        waves: test_level_waves(),
        current: 0,
        cooldown: Timer::from_seconds(1.5, TimerMode::Once),
        spawned_current: false,
    });
}

pub fn advance_waves(
    mut manager: ResMut<WaveManager>,
    enemies: Query<&Enemy>,
    time: Res<Time>,
    mut commands: Commands,
) {
    if manager.all_complete() {
        return;
    }

    // If we haven't spawned this wave yet, spawn it
    if !manager.spawned_current {
        spawn_wave(&mut commands, &manager.waves[manager.current]);
        manager.spawned_current = true;
        return;
    }

    // Wait for all enemies to die
    if enemies.iter().count() > 0 {
        return;
    }

    // Enemies cleared — tick cooldown then advance
    manager.cooldown.tick(time.delta());
    if !manager.cooldown.is_finished() {
        return;
    }

    manager.current += 1;
    manager.spawned_current = false;
    manager.cooldown.reset();
}

fn spawn_wave(commands: &mut Commands, wave: &Wave) {
    let mut idx = 0;
    let total: usize = wave.groups.iter().map(|g| g.count).sum();

    for group in &wave.groups {
        for _ in 0..group.count {
            let angle = TAU * (idx as f32) / (total as f32);
            let radius = 280.0;
            let pos = Vec2::new(angle.cos(), angle.sin()) * radius;

            let et = &group.enemy_type;
            commands.spawn((
                Sprite {
                    color: et.color(),
                    custom_size: Some(et.size()),
                    ..default()
                },
                Transform::from_translation(pos.extend(0.0)),
                Enemy,
                Health { current: et.health(), max: et.health() },
                RushBehavior { speed: et.speed() },
                DespawnOnExit(GameState::Playing),
            ));
            idx += 1;
        }
    }
}

fn test_level_waves() -> Vec<Wave> {
    vec![
        // Wave 1: just ants
        Wave {
            groups: vec![SpawnGroup {
                enemy_type: EnemyType::Ant,
                count: 5,
            }],
        },
        // Wave 2: ants + a snake
        Wave {
            groups: vec![
                SpawnGroup { enemy_type: EnemyType::Ant, count: 4 },
                SpawnGroup { enemy_type: EnemyType::GardenSnake, count: 1 },
            ],
        },
        // Wave 3: mixed, heavier
        Wave {
            groups: vec![
                SpawnGroup { enemy_type: EnemyType::Ant, count: 6 },
                SpawnGroup { enemy_type: EnemyType::GardenSnake, count: 2 },
            ],
        },
        // Wave 4: snake-heavy
        Wave {
            groups: vec![
                SpawnGroup { enemy_type: EnemyType::Ant, count: 3 },
                SpawnGroup { enemy_type: EnemyType::GardenSnake, count: 3 },
            ],
        },
        // Wave 5: boss wave
        Wave {
            groups: vec![
                SpawnGroup { enemy_type: EnemyType::GardenSnake, count: 5 },
            ],
        },
    ]
}

pub fn cleanup_waves(mut commands: Commands) {
    commands.remove_resource::<WaveManager>();
}
