use bevy::prelude::*;

use super::player::Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct RushBehavior {
    pub speed: f32,
}

pub fn rush_toward_player(
    player_q: Query<&Transform, With<Player>>,
    mut enemy_q: Query<(&RushBehavior, &mut Transform), (With<Enemy>, Without<Player>)>,
    time: Res<Time>,
) {
    let Ok(player_tf) = player_q.single() else {
        return;
    };
    let player_pos = player_tf.translation.truncate();

    for (rush, mut tf) in &mut enemy_q {
        let dir = player_pos - tf.translation.truncate();
        if dir.length() > 1.0 {
            let dir = dir.normalize();
            tf.translation.x += dir.x * rush.speed * time.delta_secs();
            tf.translation.y += dir.y * rush.speed * time.delta_secs();
        }
    }
}
