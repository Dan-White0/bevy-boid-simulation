mod components;
mod despawn_boids;
mod movement;
pub mod two_dimensions;

pub use components::Boid;
pub use despawn_boids::despawn_boids;
pub use movement::move_boids;
