use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Startup,
    MainMenu,
    Simulation2D,
    Simulation3D,
}
