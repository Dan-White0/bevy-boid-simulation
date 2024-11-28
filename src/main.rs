use bevy::prelude::*;

mod gamestate;
mod view;

use gamestate::boids::move_boids;
use gamestate::state_control::{start_stop, RunSimulation};
use gamestate::two_dimensions::spawn_boids_2d;
use view::spawn_camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<RunSimulation>()
        .add_systems(Startup, (spawn_camera, spawn_boids_2d))
        .add_systems(Update, start_stop)
        .add_systems(FixedUpdate, move_boids)
        .run();
}
