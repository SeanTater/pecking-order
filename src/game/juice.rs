//! Visual juice: hit flash, knockback, screenshake, and death particles.

use bevy::prelude::*;

use crate::states::GameState;

// ── Hit Flash ────────────────────────────────────────────────────────

/// When present, overrides sprite color to white for the timer duration.
#[derive(Component)]
pub struct HitFlash {
    pub timer: Timer,
    pub original_color: Color,
}

const FLASH_DURATION: f32 = 0.08;
const FLASH_COLOR: Color = Color::srgb(3.0, 3.0, 3.0); // HDR white

pub fn apply_hit_flash(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Sprite, &mut HitFlash)>,
) {
    for (entity, mut sprite, mut flash) in &mut query {
        // Set to white on first frame
        sprite.color = FLASH_COLOR;

        flash.timer.tick(time.delta());
        if flash.timer.is_finished() {
            sprite.color = flash.original_color;
            commands.entity(entity).remove::<HitFlash>();
        }
    }
}

/// Inserts a HitFlash component, saving the current sprite color.
pub fn trigger_flash(commands: &mut Commands, entity: Entity, current_color: Color) {
    commands.entity(entity).insert(HitFlash {
        timer: Timer::from_seconds(FLASH_DURATION, TimerMode::Once),
        original_color: current_color,
    });
}

// ── Knockback ────────────────────────────────────────────────────────

/// Velocity that decays exponentially, applied additively to transform.
#[derive(Component)]
pub struct Knockback {
    pub velocity: Vec2,
}

const KNOCKBACK_FRICTION: f32 = 10.0; // higher = faster decay
const KNOCKBACK_THRESHOLD: f32 = 5.0; // remove when velocity drops below this

pub fn apply_knockback(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Knockback)>,
) {
    for (entity, mut tf, mut kb) in &mut query {
        tf.translation.x += kb.velocity.x * time.delta_secs();
        tf.translation.y += kb.velocity.y * time.delta_secs();

        // Exponential decay
        kb.velocity *= (-KNOCKBACK_FRICTION * time.delta_secs()).exp();

        if kb.velocity.length() < KNOCKBACK_THRESHOLD {
            commands.entity(entity).remove::<Knockback>();
        }
    }
}

pub fn trigger_knockback(commands: &mut Commands, entity: Entity, direction: Vec2, strength: f32) {
    commands.entity(entity).insert(Knockback {
        velocity: direction * strength,
    });
}

// ── Screenshake ──────────────────────────────────────────────────────

/// Trauma-based screenshake. Trauma decays over time; offset = trauma² × max_offset.
#[derive(Resource, Default)]
pub struct ScreenShake {
    pub trauma: f32,
    seed: f32,
}

const MAX_SHAKE_OFFSET: f32 = 8.0;
const TRAUMA_DECAY: f32 = 3.0;

impl ScreenShake {
    pub fn add_trauma(&mut self, amount: f32) {
        self.trauma = (self.trauma + amount).min(1.0);
    }
}

pub fn apply_screenshake(
    mut shake: ResMut<ScreenShake>,
    time: Res<Time>,
    mut camera_q: Query<&mut Transform, With<Camera2d>>,
) {
    let Ok(mut cam_tf) = camera_q.single_mut() else {
        return;
    };

    if shake.trauma > 0.0 {
        shake.seed += time.delta_secs() * 50.0;
        let intensity = shake.trauma * shake.trauma; // quadratic falloff
        let offset_x = (shake.seed * 1.1).sin() * MAX_SHAKE_OFFSET * intensity;
        let offset_y = (shake.seed * 1.7).cos() * MAX_SHAKE_OFFSET * intensity;

        cam_tf.translation.x += offset_x;
        cam_tf.translation.y += offset_y;

        shake.trauma = (shake.trauma - TRAUMA_DECAY * time.delta_secs()).max(0.0);
    }
}

// ── Death Particles ──────────────────────────────────────────────────

#[derive(Component)]
pub struct DeathParticle {
    pub velocity: Vec2,
    pub lifetime: Timer,
}

const PARTICLE_COUNT: usize = 5;
const PARTICLE_SPEED: f32 = 120.0;
const PARTICLE_LIFETIME: f32 = 0.4;
const PARTICLE_SIZE: f32 = 4.0;

pub fn spawn_death_particles(commands: &mut Commands, position: Vec2, color: Color) {
    for i in 0..PARTICLE_COUNT {
        let angle = std::f32::consts::TAU * (i as f32) / PARTICLE_COUNT as f32
            + (i as f32 * 1.618); // golden ratio offset for variety
        let dir = Vec2::new(angle.cos(), angle.sin());

        commands.spawn((
            Sprite {
                color,
                custom_size: Some(Vec2::splat(PARTICLE_SIZE)),
                ..default()
            },
            Transform::from_translation(position.extend(1.0)),
            DeathParticle {
                velocity: dir * PARTICLE_SPEED * (0.6 + (i as f32 * 0.1)), // varied speed
                lifetime: Timer::from_seconds(PARTICLE_LIFETIME, TimerMode::Once),
            },
            DespawnOnExit(GameState::Playing),
        ));
    }
}

pub fn update_death_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Sprite, &mut DeathParticle)>,
) {
    for (entity, mut tf, mut sprite, mut particle) in &mut query {
        particle.lifetime.tick(time.delta());

        tf.translation.x += particle.velocity.x * time.delta_secs();
        tf.translation.y += particle.velocity.y * time.delta_secs();

        // Fade out
        let alpha = 1.0 - particle.lifetime.fraction();
        sprite.color = sprite.color.with_alpha(alpha);

        // Slow down
        particle.velocity *= 0.95;

        if particle.lifetime.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
