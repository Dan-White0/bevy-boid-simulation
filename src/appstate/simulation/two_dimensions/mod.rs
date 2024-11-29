use bevy::prelude::*;

mod spawn_boids;
mod view;

use super::boids::{despawn_boids, move_boids, Boid};
use super::state_control::{start_stop, RunSimulation};
use super::AppState;

use spawn_boids::spawn_boids_2d;
use view::set_2d_simulation_view;

pub struct SimulationLoopPlugin2D;

impl Plugin for SimulationLoopPlugin2D {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            move_boids.run_if(in_state(AppState::Simulation2D)),
        )
        .init_resource::<RunSimulation>()
        .add_systems(Update, start_stop)
        .add_systems(
            OnEnter(AppState::Simulation2D),
            (spawn_boids_2d, set_2d_simulation_view),
        )
        .add_systems(OnExit(AppState::Simulation2D), despawn_boids);
    }
}
