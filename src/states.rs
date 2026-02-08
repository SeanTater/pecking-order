use bevy::prelude::*;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    Cutscene,
    Playing,
    Paused,
}

#[derive(Resource)]
pub struct CurrentLevel(pub usize);

#[derive(Resource)]
pub struct LevelSequence {
    pub stages: Vec<Stage>,
    pub current: usize,
}

pub enum Stage {
    Cutscene(String),
    Level(String),
}
