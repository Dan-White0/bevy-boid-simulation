mod boids;
mod state_control;
mod three_dimensions;
mod two_dimensions;

use state_control::RunSimulation;

use super::AppState;

pub use three_dimensions::SimulationLoopPlugin3D;
pub use two_dimensions::SimulationLoopPlugin2D;
