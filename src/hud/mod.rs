//! HUD plugin — health hearts, tool slot icon, and wave counter.

mod health;
mod tool_display;
mod wave_indicator;

use bevy::prelude::*;

use crate::states::GameState;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), (health::spawn_hearts, tool_display::spawn_tool_display, wave_indicator::spawn_wave_indicator))
            .add_systems(
                Update,
                (health::update_hearts, tool_display::update_tool_display, wave_indicator::update_wave_indicator)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
