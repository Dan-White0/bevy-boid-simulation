// Boid settings
pub const BOID_SIZE_SCALE: f32 = 5.;
pub const BOID_HEIGHT: f32 = 3. * BOID_SIZE_SCALE;
pub const BOID_WIDTH: f32 = 2. * BOID_SIZE_SCALE;

// Simulation settings
pub const NUMBER_OF_BOIDS: usize = 200;

pub const RIGHT_MARGIN: f32 = 75.;
pub const LEFT_MARGIN: f32 = 75.;
pub const TOP_MARGIN: f32 = 75.;
pub const BOTTOM_MARGIN: f32 = 75.;
pub const DISTANT_MARGIN: f32 = 75.;
pub const CLOSE_MARGIN: f32 = 75.;
pub const TURN_FACTOR: f32 = 2.;

pub const BOID_VIEW_ANGLE_RAD: f32 = std::f32::consts::PI * 3. / 4.;
pub const VISUAL_RANGE: f32 = 80.;
pub const VISUAL_RANGE_SQUARED: f32 = VISUAL_RANGE * VISUAL_RANGE;
pub const MATCHING_FACTOR: f32 = 0.2; // How much a boid tries to match the speed of neighbouring boids
pub const CENTERING_FACTOR: f32 = 0.005; // How much a boid tends to the center of mass of nearby boids

pub const PROTECTED_RANGE: f32 = 20.;
pub const PROTECTED_RANGE_SQUARED: f32 = PROTECTED_RANGE * PROTECTED_RANGE;
pub const AVOID_FACTOR: f32 = 0.075; // How much a boid tries to avoid hitting boids that are too close

pub const MAX_SPEED: f32 = 6.;
pub const MIN_SPEED: f32 = 3.;

// Debug settings
pub const SHOW_VIEW_CONE: bool = false;
