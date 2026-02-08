use bevy::prelude::*;

use crate::game::player::Player;
use crate::game::tools::HeldTool;
use crate::states::GameState;

#[derive(Component)]
pub struct ToolSlotIcon;

#[derive(Component)]
pub struct ToolSlotImage;

pub fn spawn_tool_display(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(16.0),
            top: Val::Px(16.0),
            width: Val::Px(48.0),
            height: Val::Px(48.0),
            border: UiRect::all(Val::Px(2.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        BorderColor::all(Color::srgb(0.5, 0.5, 0.5)),
        ToolSlotIcon,
        DespawnOnExit(GameState::Playing),
    )).with_children(|parent| {
        parent.spawn((
            ImageNode {
                image: Handle::default(),
                ..default()
            },
            Node {
                width: Val::Px(36.0),
                height: Val::Px(36.0),
                ..default()
            },
            Visibility::Hidden,
            ToolSlotImage,
        ));
    });
}

pub fn update_tool_display(
    player_q: Query<Option<&HeldTool>, With<Player>>,
    mut image_q: Query<(&mut ImageNode, &mut Visibility), With<ToolSlotImage>>,
    asset_server: Res<AssetServer>,
) {
    let Ok(held) = player_q.single() else {
        return;
    };
    let Ok((mut image_node, mut vis)) = image_q.single_mut() else {
        return;
    };

    match held {
        Some(tool) => {
            image_node.image = asset_server.load(tool.0.sprite_path());
            *vis = Visibility::Inherited;
        }
        None => {
            *vis = Visibility::Hidden;
        }
    }
}
