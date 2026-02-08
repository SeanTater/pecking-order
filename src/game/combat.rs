//! Damage pipeline: auto-peck, contact damage, damage events, health, death.

use bevy::prelude::*;

use crate::states::GameState;
use super::enemy::Enemy;
use super::player::Player;

#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Component)]
pub struct AutoPeck {
    pub range: f32,
    pub cooldown: Timer,
    pub damage: i32,
}

/// Brief invincibility after taking contact damage.
#[derive(Component)]
pub struct IFrames {
    pub timer: Timer,
}

#[derive(Message)]
pub struct DamageEvent {
    pub target: Entity,
    pub amount: i32,
}

pub fn auto_peck(
    mut peck_q: Query<(&Transform, &mut AutoPeck), With<Player>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    time: Res<Time>,
    mut damage_events: MessageWriter<DamageEvent>,
) {
    let Ok((player_tf, mut peck)) = peck_q.single_mut() else {
        return;
    };

    peck.cooldown.tick(time.delta());
    if !peck.cooldown.is_finished() {
        return;
    }

    let player_pos = player_tf.translation.truncate();
    let mut nearest: Option<(Entity, f32)> = None;

    for (entity, tf) in &enemies {
        let dist = player_pos.distance(tf.translation.truncate());
        if dist <= peck.range {
            if nearest.is_none() || dist < nearest.unwrap().1 {
                nearest = Some((entity, dist));
            }
        }
    }

    if let Some((target, _)) = nearest {
        peck.cooldown.reset();
        damage_events.write(DamageEvent {
            target,
            amount: peck.damage,
        });
    }
}

pub fn tick_iframes(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut IFrames)>,
) {
    for (entity, mut iframes) in &mut query {
        iframes.timer.tick(time.delta());
        if iframes.timer.is_finished() {
            commands.entity(entity).remove::<IFrames>();
        }
    }
}

pub fn enemy_contact_damage(
    enemies: Query<&Transform, With<Enemy>>,
    mut commands: Commands,
    player_q: Query<(Entity, &Transform), (With<Player>, Without<IFrames>)>,
    mut damage_events: MessageWriter<DamageEvent>,
) {
    let Ok((player_entity, player_tf)) = player_q.single() else {
        return; // no player, or player has iframes
    };
    let player_pos = player_tf.translation.truncate();
    let contact_range = 20.0;

    for enemy_tf in &enemies {
        if player_pos.distance(enemy_tf.translation.truncate()) < contact_range {
            damage_events.write(DamageEvent {
                target: player_entity,
                amount: 1,
            });
            // Grant 0.5s of invincibility
            commands.entity(player_entity).insert(IFrames {
                timer: Timer::from_seconds(0.5, TimerMode::Once),
            });
            break;
        }
    }
}

pub fn apply_damage(
    mut events: MessageReader<DamageEvent>,
    mut health_q: Query<&mut Health>,
) {
    for ev in events.read() {
        if let Ok(mut health) = health_q.get_mut(ev.target) {
            health.current -= ev.amount;
        }
    }
}

pub fn check_death(
    mut commands: Commands,
    query: Query<(Entity, &Health, Option<&Player>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (entity, health, is_player) in &query {
        if health.current <= 0 {
            if is_player.is_some() {
                next_state.set(GameState::MainMenu);
                return;
            }
            commands.entity(entity).despawn();
        }
    }
}
