//! Tool system: ground items with bobble, pickup/swap (E key), and per-tool activation.

use bevy::prelude::*;

use crate::states::GameState;
use super::combat::DamageEvent;
use super::enemy::Enemy;
use super::player::Player;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ToolKind {
    Pinecone,
}

impl ToolKind {
    pub fn color(&self) -> Color {
        match self {
            ToolKind::Pinecone => Color::srgb(0.55, 0.35, 0.15),
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            ToolKind::Pinecone => "Pinecone",
        }
    }
}

/// Marker for items sitting on the ground.
#[derive(Component)]
pub struct GroundItem {
    pub kind: ToolKind,
    pub base_y: f32,
}

/// Sine-wave bobble state.
#[derive(Component)]
pub struct Bobble {
    pub elapsed: f32,
}

/// Attached to the player when they hold a tool.
#[derive(Component)]
pub struct HeldTool(pub ToolKind);

/// In-flight pinecone projectile.
#[derive(Component)]
pub struct PineconeProjectile {
    pub direction: Vec2,
    pub speed: f32,
    pub distance_left: f32,
}

const BOBBLE_AMPLITUDE: f32 = 3.0;
const BOBBLE_SPEED: f32 = 3.0;
const PICKUP_RANGE: f32 = 48.0;
const PINECONE_SPEED: f32 = 350.0;
const PINECONE_RANGE: f32 = 200.0;
const PINECONE_BLAST_RADIUS: f32 = 60.0;
const PINECONE_DAMAGE: i32 = 5;

pub fn bobble_items(
    time: Res<Time>,
    mut query: Query<(&GroundItem, &mut Bobble, &mut Transform)>,
) {
    for (ground, mut bobble, mut tf) in &mut query {
        bobble.elapsed += time.delta_secs();
        tf.translation.y = ground.base_y + (bobble.elapsed * BOBBLE_SPEED).sin() * BOBBLE_AMPLITUDE;
    }
}

pub fn pickup_tool(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    player_q: Query<(Entity, &Transform, Option<&HeldTool>), With<Player>>,
    ground_q: Query<(Entity, &Transform, &GroundItem)>,
) {
    if !keyboard.just_pressed(KeyCode::KeyE) {
        return;
    }
    let Ok((player_entity, player_tf, held)) = player_q.single() else {
        return;
    };
    let player_pos = player_tf.translation.truncate();

    // Find nearest ground item in range
    let mut nearest: Option<(Entity, f32, ToolKind)> = None;
    for (entity, tf, ground) in &ground_q {
        let dist = player_pos.distance(tf.translation.truncate());
        if dist <= PICKUP_RANGE {
            if nearest.is_none() || dist < nearest.unwrap().1 {
                nearest = Some((entity, dist, ground.kind));
            }
        }
    }

    let Some((item_entity, _, new_kind)) = nearest else {
        return;
    };

    // If already holding a tool, drop it at player position
    if let Some(old_tool) = held {
        let old_kind = old_tool.0;
        commands.entity(player_entity).remove::<HeldTool>();
        spawn_ground_item(&mut commands, old_kind, player_pos);
    }

    // Pick up new tool
    commands.entity(item_entity).despawn();
    commands.entity(player_entity).insert(HeldTool(new_kind));
}

pub fn activate_tool(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    player_q: Query<(Entity, &Transform, &HeldTool), With<Player>>,
    enemies: Query<&Transform, With<Enemy>>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }
    let Ok((player_entity, player_tf, held)) = player_q.single() else {
        return;
    };

    match held.0 {
        ToolKind::Pinecone => {
            let player_pos = player_tf.translation.truncate();

            // Aim toward nearest enemy, or default to right
            let direction = enemies
                .iter()
                .map(|tf| tf.translation.truncate())
                .min_by(|a, b| {
                    a.distance(player_pos)
                        .partial_cmp(&b.distance(player_pos))
                        .unwrap()
                })
                .map(|target| (target - player_pos).normalize_or_zero())
                .unwrap_or(Vec2::X);

            commands.spawn((
                Sprite {
                    color: ToolKind::Pinecone.color(),
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..default()
                },
                Transform::from_translation(player_pos.extend(0.0)),
                PineconeProjectile {
                    direction,
                    speed: PINECONE_SPEED,
                    distance_left: PINECONE_RANGE,
                },
                DespawnOnExit(GameState::Playing),
            ));

            // Consume the tool
            commands.entity(player_entity).remove::<HeldTool>();
        }
    }
}

pub fn pinecone_fly(
    mut commands: Commands,
    time: Res<Time>,
    mut projectiles: Query<(Entity, &mut Transform, &mut PineconeProjectile)>,
    enemies: Query<(Entity, &Transform), (With<Enemy>, Without<PineconeProjectile>)>,
    mut damage_events: MessageWriter<DamageEvent>,
) {
    for (proj_entity, mut tf, mut proj) in &mut projectiles {
        let move_dist = proj.speed * time.delta_secs();
        tf.translation.x += proj.direction.x * move_dist;
        tf.translation.y += proj.direction.y * move_dist;
        proj.distance_left -= move_dist;

        if proj.distance_left <= 0.0 {
            // Explode — AoE damage
            let blast_pos = tf.translation.truncate();
            for (enemy_entity, enemy_tf) in &enemies {
                if blast_pos.distance(enemy_tf.translation.truncate()) <= PINECONE_BLAST_RADIUS {
                    damage_events.write(DamageEvent {
                        target: enemy_entity,
                        amount: PINECONE_DAMAGE,
                    });
                }
            }
            commands.entity(proj_entity).despawn();
        }
    }
}

fn spawn_ground_item(commands: &mut Commands, kind: ToolKind, pos: Vec2) {
    commands.spawn((
        Sprite {
            color: kind.color(),
            custom_size: Some(Vec2::new(12.0, 12.0)),
            ..default()
        },
        Transform::from_translation(pos.extend(0.0)),
        GroundItem {
            kind,
            base_y: pos.y,
        },
        Bobble { elapsed: 0.0 },
        DespawnOnExit(GameState::Playing),
    ));
}

pub fn spawn_ground_items(mut commands: Commands) {
    // Drop a couple pinecones near spawn for testing
    spawn_ground_item(&mut commands, ToolKind::Pinecone, Vec2::new(60.0, 30.0));
    spawn_ground_item(&mut commands, ToolKind::Pinecone, Vec2::new(-80.0, 50.0));
}
