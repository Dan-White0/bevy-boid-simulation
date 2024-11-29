use bevy::prelude::*;

mod appstate;
mod camera;
mod configuration;

use appstate::main_menu::MainMenuPlugin;
use appstate::simulation::SimulationLoopPlugin2D;
use appstate::simulation::SimulationLoopPlugin3D;
use appstate::AppState;
use camera::spawn_camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MainMenuPlugin)
        .add_plugins(SimulationLoopPlugin2D)
        .add_plugins(SimulationLoopPlugin3D)
        .init_state::<AppState>()
        .add_systems(Startup, spawn_camera)
        .run();
}
