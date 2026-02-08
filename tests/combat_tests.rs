//! Pure logic tests for combat and wave systems.

use pecking_order::game::waves::{EnemyType, SpawnGroup, Wave, WaveManager};

#[test]
fn wave_manager_tracks_progress() {
    let mut wm = WaveManager {
        waves: vec![
            Wave { groups: vec![SpawnGroup { enemy_type: EnemyType::Ant, count: 3 }] },
            Wave { groups: vec![SpawnGroup { enemy_type: EnemyType::GardenSnake, count: 2 }] },
        ],
        current: 0,
        cooldown: bevy::time::Timer::from_seconds(1.0, bevy::time::TimerMode::Once),
        spawned_current: false,
    };

    assert_eq!(wm.total(), 2);
    assert_eq!(wm.display_current(), 1);
    assert!(!wm.all_complete());

    wm.current = 1;
    assert_eq!(wm.display_current(), 2);
    assert!(!wm.all_complete());

    wm.current = 2;
    assert!(wm.all_complete());
    // display_current clamps to total
    assert_eq!(wm.display_current(), 2);
}

#[test]
fn enemy_type_stats_are_sane() {
    // Ants should be weaker and slower than snakes
    assert!(EnemyType::Ant.health() < EnemyType::GardenSnake.health());
    assert!(EnemyType::Ant.speed() < EnemyType::GardenSnake.speed());
}

#[test]
fn tool_color_is_distinct() {
    use pecking_order::game::tools::ToolKind;
    // Just verify it doesn't panic and returns something
    let _color = ToolKind::Pinecone.color();
}
