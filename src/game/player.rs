//! Player movement and sprite animation.

use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Speed(pub f32);

/// Tracks which direction the player last moved, for sprite flipping.
#[derive(Component)]
pub struct Facing {
    pub right: bool,
}

/// Cycles through sprite frames when moving.
#[derive(Component)]
pub struct WalkAnimation {
    pub standing: Handle<Image>,
    pub pecking: Handle<Image>,
    pub frames: Vec<Handle<Image>>,
    pub timer: Timer,
    pub current_frame: usize,
    pub moving: bool,
}

/// Brief override to show the peck sprite when auto-peck fires.
#[derive(Component)]
pub struct PeckFlash {
    pub timer: Timer,
}

const PECK_FLASH_DURATION: f32 = 0.12;

pub fn trigger_peck_flash(commands: &mut Commands, entity: Entity) {
    commands.entity(entity).insert(PeckFlash {
        timer: Timer::from_seconds(PECK_FLASH_DURATION, TimerMode::Once),
    });
}

pub fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Speed, &mut Transform, &mut Facing, &mut WalkAnimation, &mut Sprite), With<Player>>,
) {
    let Ok((speed, mut transform, mut facing, mut anim, mut sprite)) = query.single_mut() else {
        return;
    };

    let mut dir = Vec2::ZERO;
    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        dir.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        dir.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        dir.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        dir.x += 1.0;
    }

    if dir != Vec2::ZERO {
        dir = dir.normalize();
        transform.translation.x += dir.x * speed.0 * time.delta_secs();
        transform.translation.y += dir.y * speed.0 * time.delta_secs();

        // Flip sprite based on horizontal direction
        if dir.x > 0.1 {
            facing.right = true;
        } else if dir.x < -0.1 {
            facing.right = false;
        }
        anim.moving = true;
    } else {
        anim.moving = false;
    }

    // Flip sprite
    sprite.flip_x = !facing.right;
}

pub fn animate_player(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut WalkAnimation, &mut Sprite, Option<&mut PeckFlash>), With<Player>>,
) {
    let Ok((entity, mut anim, mut sprite, peck_flash)) = query.single_mut() else {
        return;
    };

    // Peck animation takes priority
    if let Some(mut flash) = peck_flash {
        flash.timer.tick(time.delta());
        sprite.image = anim.pecking.clone();
        if flash.timer.is_finished() {
            commands.entity(entity).remove::<PeckFlash>();
        }
        return;
    }

    if anim.moving {
        anim.timer.tick(time.delta());
        if anim.timer.just_finished() {
            anim.current_frame = (anim.current_frame + 1) % anim.frames.len();
        }
        sprite.image = anim.frames[anim.current_frame].clone();
    } else {
        sprite.image = anim.standing.clone();
        anim.current_frame = 0;
    }
}
