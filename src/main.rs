use bevy::prelude::*;

mod appstate;
mod configuration;
mod view;

use appstate::main_menu::MainMenuPlugin;
use appstate::simulation::SimulationLoopPlugin;
use appstate::AppState;
use view::spawn_camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MainMenuPlugin)
        .add_plugins(SimulationLoopPlugin)
        .init_state::<AppState>()
        .add_systems(Startup, spawn_camera)
        .run();
}
