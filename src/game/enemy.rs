use bevy::prelude::*;

use super::player::Player;

#[derive(Component)]
pub struct Enemy;

/// Color used for death particles (since image sprites have white tint).
#[derive(Component)]
pub struct DeathColor(pub Color);

#[derive(Component)]
pub struct RushBehavior {
    pub speed: f32,
}

pub fn rush_toward_player(
    player_q: Query<&Transform, With<Player>>,
    mut enemy_q: Query<(&RushBehavior, &mut Transform, &mut Sprite), (With<Enemy>, Without<Player>)>,
    time: Res<Time>,
) {
    let Ok(player_tf) = player_q.single() else {
        return;
    };
    let player_pos = player_tf.translation.truncate();

    for (rush, mut tf, mut sprite) in &mut enemy_q {
        let dir = player_pos - tf.translation.truncate();
        if dir.length() > 1.0 {
            let norm = dir.normalize();
            tf.translation.x += norm.x * rush.speed * time.delta_secs();
            tf.translation.y += norm.y * rush.speed * time.delta_secs();
            // Face toward player
            sprite.flip_x = norm.x < 0.0;
        }
    }
}
