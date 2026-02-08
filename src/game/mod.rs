//! Core gameplay plugin — wires player, enemies, combat, tools, and waves.

mod camera;
pub mod combat;
pub mod enemy;
pub mod juice;
pub mod player;
pub mod tools;
pub mod waves;

use bevy::prelude::*;

use crate::states::GameState;
use combat::{AutoPeck, DamageEvent, Health};
use player::{Player, Speed, Facing, WalkAnimation};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<juice::ScreenShake>()
            .add_message::<DamageEvent>()
            .add_systems(OnEnter(GameState::Playing), (setup_playing, waves::init_waves, tools::spawn_ground_items).chain())
            .add_systems(OnExit(GameState::Playing), waves::cleanup_waves)
            .add_systems(
                Update,
                (
                    (player::move_player, player::animate_player, camera::camera_follow, enemy::rush_toward_player, waves::advance_waves),
                    (combat::tick_iframes, combat::auto_peck, combat::enemy_contact_damage),
                    combat::apply_damage,
                    combat::check_death,
                    (tools::bobble_items, tools::pickup_tool, tools::activate_tool, tools::pinecone_fly),
                    (juice::apply_hit_flash, juice::apply_knockback, juice::apply_screenshake, juice::update_death_particles),
                    back_to_menu,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

const PLAYER_SIZE: Vec2 = Vec2::new(40.0, 40.0);

fn setup_playing(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        DespawnOnExit(GameState::Playing),
    ));

    // Green background
    commands.spawn((
        Sprite {
            color: Color::srgb(0.35, 0.55, 0.25),
            custom_size: Some(Vec2::new(2000.0, 2000.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, -10.0)),
        DespawnOnExit(GameState::Playing),
    ));

    // Player bird — Pip
    let standing = asset_server.load("pip/pip-standing.webp");
    commands.spawn((
        Sprite {
            image: standing.clone(),
            custom_size: Some(PLAYER_SIZE),
            ..default()
        },
        Transform::from_translation(Vec3::ZERO),
        Player,
        Speed(200.0),
        Facing { right: true },
        WalkAnimation {
            standing,
            pecking: asset_server.load("pip/pip-pecking.webp"),
            frames: vec![
                asset_server.load("pip/pip-walking-1.webp"),
                asset_server.load("pip/pip-standing.webp"),
                asset_server.load("pip/pip-walking-2.webp"),
                asset_server.load("pip/pip-standing.webp"),
            ],
            timer: Timer::from_seconds(0.15, TimerMode::Repeating),
            current_frame: 0,
            moving: false,
        },
        Health { current: 10, max: 10 },
        AutoPeck {
            range: 50.0,
            cooldown: Timer::from_seconds(0.4, TimerMode::Once),
            damage: 1,
        },
        DespawnOnExit(GameState::Playing),
    ));
}

fn back_to_menu(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
    }
}
