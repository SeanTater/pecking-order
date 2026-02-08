//! Damage pipeline: auto-peck, contact damage, damage events, health, death.

use bevy::prelude::*;

use crate::states::GameState;
use super::enemy::{Enemy, DeathColor};
use super::juice::{self, ScreenShake};
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
    mut commands: Commands,
    mut peck_q: Query<(Entity, &Transform, &mut AutoPeck), With<Player>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    time: Res<Time>,
    mut damage_events: MessageWriter<DamageEvent>,
) {
    let Ok((player_entity, player_tf, mut peck)) = peck_q.single_mut() else {
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
        super::player::trigger_peck_flash(&mut commands, player_entity);
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
    mut commands: Commands,
    mut events: MessageReader<DamageEvent>,
    mut health_q: Query<(&mut Health, &Sprite, &Transform, Option<&Player>)>,
    player_q: Query<&Transform, With<Player>>,
    mut shake: ResMut<ScreenShake>,
) {
    let player_pos = player_q.single().map(|t| t.translation.truncate()).unwrap_or(Vec2::ZERO);

    for ev in events.read() {
        if let Ok((mut health, sprite, tf, is_player)) = health_q.get_mut(ev.target) {
            health.current -= ev.amount;

            // Hit flash
            juice::trigger_flash(&mut commands, ev.target, sprite.color);

            // Knockback — away from player for enemies, away from enemy center for player
            let pos = tf.translation.truncate();
            if is_player.is_some() {
                // Player got hit — knock away from center of enemies (approximate with zero for now)
                let kb_dir = (pos - Vec2::ZERO).normalize_or_zero();
                juice::trigger_knockback(&mut commands, ev.target, kb_dir, 200.0);
                shake.add_trauma(0.3);
            } else {
                // Enemy got hit — knock away from player
                let kb_dir = (pos - player_pos).normalize_or_zero();
                juice::trigger_knockback(&mut commands, ev.target, kb_dir, 150.0);
                shake.add_trauma(0.05);
            }
        }
    }
}

pub fn check_death(
    mut commands: Commands,
    query: Query<(Entity, &Health, &Transform, Option<&Player>, Option<&DeathColor>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut shake: ResMut<ScreenShake>,
) {
    for (entity, health, tf, is_player, death_color) in &query {
        if health.current <= 0 {
            if is_player.is_some() {
                next_state.set(GameState::MainMenu);
                return;
            }
            let color = death_color.map(|dc| dc.0).unwrap_or(Color::srgb(0.5, 0.5, 0.5));
            juice::spawn_death_particles(&mut commands, tf.translation.truncate(), color);
            shake.add_trauma(0.1);
            commands.entity(entity).despawn();
        }
    }
}
