use bevy::prelude::*;
mod boids;
mod state_control;

use super::AppState;
use boids::{despawn_boids, move_boids, two_dimensions::spawn_boids_2d};
use state_control::{start_stop, RunSimulation};

pub struct SimulationLoopPlugin;

impl Plugin for SimulationLoopPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            move_boids.run_if(in_state(AppState::Simulation)),
        )
        .init_resource::<RunSimulation>()
        .add_systems(Update, start_stop)
        .add_systems(OnEnter(AppState::Simulation), spawn_boids_2d)
        .add_systems(OnExit(AppState::Simulation), despawn_boids);
    }
}
