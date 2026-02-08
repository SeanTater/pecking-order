use bevy::prelude::*;

use crate::game::player::Player;
use crate::game::tools::HeldTool;
use crate::states::GameState;

#[derive(Component)]
pub struct ToolSlotIcon;

pub fn spawn_tool_display(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(16.0),
            top: Val::Px(16.0),
            width: Val::Px(40.0),
            height: Val::Px(40.0),
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
        BorderColor::all(Color::srgb(0.6, 0.6, 0.6)),
        ToolSlotIcon,
        DespawnOnExit(GameState::Playing),
    ));
}

pub fn update_tool_display(
    player_q: Query<Option<&HeldTool>, With<Player>>,
    mut icon_q: Query<&mut BackgroundColor, With<ToolSlotIcon>>,
) {
    let Ok(held) = player_q.single() else {
        return;
    };
    let Ok(mut bg) = icon_q.single_mut() else {
        return;
    };

    match held {
        Some(tool) => *bg = BackgroundColor(tool.0.color()),
        None => *bg = BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
    }
}
