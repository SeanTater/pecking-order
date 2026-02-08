//! Wave system: authored sequences of enemy spawns with breathing room between waves.

use bevy::prelude::*;
use std::f32::consts::TAU;

use crate::states::GameState;
use super::combat::Health;
use super::enemy::{Enemy, DeathColor, RushBehavior};
use super::tools;

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

    pub fn size(&self) -> Vec2 {
        match self {
            EnemyType::Ant => Vec2::new(20.0, 16.0),
            EnemyType::GardenSnake => Vec2::new(48.0, 20.0),
        }
    }

    const ANT_SPRITES: &[&'static str] = &[
        "ants/ant.webp",
        "ants/ant-with-leaf.webp",
        "ants/ant-with-stick.webp",
        "ants/ant-with-blueberry.webp",
    ];

    pub fn sprite_path(&self, index: usize) -> &'static str {
        match self {
            EnemyType::Ant => Self::ANT_SPRITES[index % Self::ANT_SPRITES.len()],
            EnemyType::GardenSnake => "noodle/noodle-slithering.webp",
        }
    }

    /// Color used for death particles.
    pub fn color(&self) -> Color {
        match self {
            EnemyType::Ant => Color::srgb(0.45, 0.3, 0.15),
            EnemyType::GardenSnake => Color::srgb(0.4, 0.6, 0.25),
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
    asset_server: Res<AssetServer>,
) {
    if manager.all_complete() {
        return;
    }

    // If we haven't spawned this wave yet, spawn it
    if !manager.spawned_current {
        spawn_wave(&mut commands, &asset_server, &manager.waves[manager.current]);
        // Drop a pinecone every other wave (starting wave 2)
        if manager.current > 0 && manager.current % 2 == 1 {
            let angle = (manager.current as f32) * 1.618 * TAU;
            let pos = Vec2::new(angle.cos(), angle.sin()) * 100.0;
            tools::spawn_ground_item_pub(&mut commands, &asset_server, tools::ToolKind::Pinecone, pos);
        }
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

fn spawn_wave(commands: &mut Commands, asset_server: &AssetServer, wave: &Wave) {
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
                    image: asset_server.load(et.sprite_path(idx)),
                    custom_size: Some(et.size()),
                    ..default()
                },
                Transform::from_translation(pos.extend(0.0)),
                Enemy,
                DeathColor(et.color()),
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
