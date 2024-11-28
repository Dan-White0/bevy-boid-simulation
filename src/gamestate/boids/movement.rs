use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::window::PrimaryWindow;

use super::Boid;
use crate::RunSimulation;

const LEFT_MARGIN: f32 = 75.;
const RIGHT_MARGIN: f32 = 75.;
const TOP_MARGIN: f32 = 75.;
const BOTTOM_MARGIN: f32 = 75.;

const TURN_FACTOR: f32 = 0.5;
const VISUAL_RANGE: f32 = 80.;
const VISUAL_RANGE_SQUARED: f32 = VISUAL_RANGE * VISUAL_RANGE;
const PROTECTED_RANGE: f32 = 20.;
const PROTECTED_RANGE_SQUARED: f32 = PROTECTED_RANGE * PROTECTED_RANGE;
const CENTERING_FACTOR: f32 = 0.005;
const AVOID_FACTOR: f32 = 0.075;
const MATCHING_FACTOR: f32 = 0.2;
const MAX_SPEED: f32 = 6.;
const MIN_SPEED: f32 = 3.;

pub fn move_boids(
    mut boid_query: Query<(&mut Transform, Entity, &mut Boid)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    run_simulation: ResMut<RunSimulation>,
) {
    if !run_simulation.0 {
        return;
    }

    let window = window_query.get_single().unwrap();
    let mut movement_vectors: HashMap<Entity, Vec3> = HashMap::new();

    for (boid_a_transform, boid_a_entity, boid_a) in boid_query.iter() {
        let mut pos_avg = Vec3::ZERO;
        let mut vel_avg = Vec3::ZERO;
        let mut close_distance = Vec3::ZERO;

        let mut neighboring_boids = 0.;

        for (boid_b_transform, boid_b_entity, boid_b) in boid_query.iter() {
            if boid_a_entity == boid_b_entity {
                continue;
            }

            let vector_between = boid_a_transform.translation - boid_b_transform.translation;

            if vector_between.x.abs() < VISUAL_RANGE && vector_between.y.abs() < VISUAL_RANGE {
                let squared_distance = vector_between.distance_squared(Vec3::ZERO);

                if squared_distance < PROTECTED_RANGE_SQUARED {
                    close_distance += vector_between;
                } else if squared_distance < VISUAL_RANGE_SQUARED {
                    pos_avg += boid_b_transform.translation;
                    vel_avg += boid_b.velocity;

                    neighboring_boids += 1.;
                }
            }
        }

        let mut new_velocity = boid_a.velocity;

        if neighboring_boids > 0. {
            pos_avg /= neighboring_boids;
            vel_avg /= neighboring_boids;

            new_velocity += new_velocity
                + (pos_avg - boid_a_transform.translation) * CENTERING_FACTOR
                + (vel_avg - new_velocity) * MATCHING_FACTOR;
        }

        new_velocity += close_distance * AVOID_FACTOR;

        if boid_a_transform.translation.x < LEFT_MARGIN {
            new_velocity += Vec3::new(TURN_FACTOR, 0., 0.);
        }
        if boid_a_transform.translation.x > window.width() - RIGHT_MARGIN {
            new_velocity -= Vec3::new(TURN_FACTOR, 0., 0.);
        }
        if boid_a_transform.translation.y < TOP_MARGIN {
            new_velocity += Vec3::new(0., TURN_FACTOR, 0.);
        }
        if boid_a_transform.translation.y > window.height() - BOTTOM_MARGIN {
            new_velocity -= Vec3::new(0., TURN_FACTOR, 0.);
        }

        let mut movement_vector = new_velocity;
        let speed = movement_vector.length();

        if speed < MIN_SPEED {
            movement_vector *= MIN_SPEED / speed;
        }
        if speed > MAX_SPEED {
            movement_vector *= MAX_SPEED / speed;
        }

        movement_vectors.insert(boid_a_entity, movement_vector);
    }

    for (mut boid_transform, boid_entity, mut boid) in boid_query.iter_mut() {
        let movement_vector = *movement_vectors
            .get(&boid_entity)
            .expect("All boids should have a movement vector in the map");

        boid_transform.translation += movement_vector;
        boid.velocity = movement_vector;

        boid_transform.rotation = Quat::from_rotation_arc(Vec3::Y, movement_vector.normalize());
    }
}
