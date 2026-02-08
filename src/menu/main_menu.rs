use bevy::{app::AppExit, prelude::*};

use crate::states::GameState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(
                Update,
                (button_system, button_action).run_if(in_state(GameState::MainMenu)),
            );
    }
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

#[derive(Component)]
enum MenuButtonAction {
    Play,
    Quit,
}

fn setup_main_menu(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        DespawnOnExit(GameState::MainMenu),
    ));

    let button_node = Node {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(10.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    commands.spawn((
        DespawnOnExit(GameState::MainMenu),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (
                Text::new("Pecking Order"),
                TextFont {
                    font_size: 64.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                Node {
                    margin: UiRect::all(Val::Px(40.0)),
                    ..default()
                },
            ),
            (
                Button,
                button_node.clone(),
                BackgroundColor(NORMAL_BUTTON),
                MenuButtonAction::Play,
                children![(
                    Text::new("Play"),
                    TextFont { font_size: 33.0, ..default() },
                    TextColor(TEXT_COLOR),
                )]
            ),
            (
                Button,
                button_node,
                BackgroundColor(NORMAL_BUTTON),
                MenuButtonAction::Quit,
                children![(
                    Text::new("Quit"),
                    TextFont { font_size: 33.0, ..default() },
                    TextColor(TEXT_COLOR),
                )]
            )
        ],
    ));
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => *color = PRESSED_BUTTON.into(),
            Interaction::Hovered => *color = HOVERED_BUTTON.into(),
            Interaction::None => *color = NORMAL_BUTTON.into(),
        }
    }
}

fn button_action(
    interaction_query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
                MenuButtonAction::Play => next_state.set(GameState::Playing),
                MenuButtonAction::Quit => {
                    exit.write(AppExit::Success);
                }
            }
        }
    }
}
