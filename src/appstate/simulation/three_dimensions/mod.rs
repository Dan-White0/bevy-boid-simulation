use bevy::prelude::*;

mod parameters;
mod spawn_boids;
mod view;

use super::boids::{despawn_boids, move_boids, Boid};
use super::state_control::{start_stop, RunSimulation};
use super::AppState;
use super::BoundingBox;

use parameters::set_bounding_box_3d;
use spawn_boids::spawn_boids_3d;
use view::set_3d_simulation_view;

pub struct SimulationLoopPlugin3D;

impl Plugin for SimulationLoopPlugin3D {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            move_boids.run_if(in_state(AppState::Simulation3D)),
        )
        .init_resource::<RunSimulation>()
        .add_systems(Update, start_stop)
        .add_systems(
            OnEnter(AppState::Simulation3D),
            (set_bounding_box_3d, spawn_boids_3d, set_3d_simulation_view).chain(),
        )
        .add_systems(OnExit(AppState::Simulation3D), despawn_boids);
    }
}
