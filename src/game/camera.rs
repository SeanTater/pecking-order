use bevy::prelude::*;

use super::player::Player;

pub fn camera_follow(
    player_q: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    mut camera_q: Query<&mut Transform, With<Camera2d>>,
    time: Res<Time>,
) {
    let Ok(player_tf) = player_q.single() else {
        return;
    };
    let Ok(mut cam_tf) = camera_q.single_mut() else {
        return;
    };

    let target = player_tf.translation.truncate();
    let current = cam_tf.translation.truncate();
    let new_pos = current.lerp(target, 5.0 * time.delta_secs());
    cam_tf.translation.x = new_pos.x;
    cam_tf.translation.y = new_pos.y;
}
