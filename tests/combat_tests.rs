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

#[test]
fn health_bar_color_gradient() {
    use pecking_order::game::healthbar::health_color;

    // Full health should be greenish
    let full = health_color(1.0);
    // Half health should be yellowish
    let half = health_color(0.5);
    // Low health should be reddish
    let low = health_color(0.1);

    // Just verify they don't panic and produce distinct colors
    assert_ne!(format!("{full:?}"), format!("{half:?}"));
    assert_ne!(format!("{half:?}"), format!("{low:?}"));
}

#[test]
fn health_bar_color_at_boundaries() {
    use pecking_order::game::healthbar::health_color;

    // Edge cases shouldn't panic
    let _zero = health_color(0.0);
    let _one = health_color(1.0);
    let _mid = health_color(0.5);
}
