use bevy::prelude::*;

use pecking_order::states::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), start_loading)
            .add_systems(Update, check_loading.run_if(in_state(GameState::Loading)));
    }
}

fn start_loading(mut commands: Commands) {
    commands.insert_resource(LoadingTracker { ready: false, timer: Timer::from_seconds(0.1, TimerMode::Once) });
}

#[derive(Resource)]
struct LoadingTracker {
    ready: bool,
    timer: Timer,
}

fn check_loading(
    mut next_state: ResMut<NextState<GameState>>,
    mut tracker: ResMut<LoadingTracker>,
    time: Res<Time>,
) {
    tracker.timer.tick(time.delta());
    if tracker.timer.just_finished() && !tracker.ready {
        tracker.ready = true;
        next_state.set(GameState::MainMenu);
    }
}
