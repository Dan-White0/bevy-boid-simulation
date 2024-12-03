use bevy::prelude::*;

mod appstate;
mod configuration;

use appstate::main_menu::MainMenuPlugin;
use appstate::simulation::BoundingBox;
use appstate::simulation::SimulationLoopPlugin2D;
use appstate::simulation::SimulationLoopPlugin3D;
use appstate::startup::StartupPlugin;
use appstate::AppState;

fn main() {
    App::new()
        .init_resource::<BoundingBox>()
        .add_plugins(DefaultPlugins)
        .add_plugins(StartupPlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(SimulationLoopPlugin2D)
        .add_plugins(SimulationLoopPlugin3D)
        .init_state::<AppState>()
        .run();
}
