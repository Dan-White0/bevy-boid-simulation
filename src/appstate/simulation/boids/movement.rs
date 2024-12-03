use bevy::prelude::*;
use bevy::utils::HashMap;

use super::super::RunSimulation;
use super::Boid;
use crate::appstate::simulation::BoundingBox;
use crate::configuration::config;

pub fn move_boids(
    mut boid_query: Query<(&mut Transform, Entity, &mut Boid)>,
    bounding_box: Res<BoundingBox>,
    run_simulation: Res<RunSimulation>,
) {
    if !run_simulation.0 {
        return;
    }

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

            if vector_between.x.abs() < config::VISUAL_RANGE
                && vector_between.y.abs() < config::VISUAL_RANGE
            {
                let squared_distance = vector_between.distance_squared(Vec3::ZERO);

                if squared_distance < config::PROTECTED_RANGE_SQUARED {
                    close_distance += vector_between;
                } else if squared_distance < config::VISUAL_RANGE_SQUARED {
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
                + (pos_avg - boid_a_transform.translation) * config::CENTERING_FACTOR
                + (vel_avg - new_velocity) * config::MATCHING_FACTOR;
        }

        new_velocity += close_distance * config::AVOID_FACTOR;

        if boid_a_transform.translation.x < config::LEFT_MARGIN {
            new_velocity += Vec3::new(config::TURN_FACTOR, 0., 0.);
        }
        if boid_a_transform.translation.x > bounding_box.0.x - config::RIGHT_MARGIN {
            new_velocity -= Vec3::new(config::TURN_FACTOR, 0., 0.);
        }
        if boid_a_transform.translation.y < config::TOP_MARGIN {
            new_velocity += Vec3::new(0., config::TURN_FACTOR, 0.);
        }
        if boid_a_transform.translation.y > bounding_box.0.y - config::BOTTOM_MARGIN {
            new_velocity -= Vec3::new(0., config::TURN_FACTOR, 0.);
        }
        if boid_a_transform.translation.z < config::DISTANT_MARGIN {
            new_velocity += Vec3::new(0., 0., config::TURN_FACTOR);
        }
        if boid_a_transform.translation.z > bounding_box.0.z - config::CLOSE_MARGIN {
            new_velocity -= Vec3::new(0., 0., config::TURN_FACTOR);
        }

        let mut movement_vector = new_velocity;
        let speed = movement_vector.length();

        if speed < config::MIN_SPEED {
            movement_vector *= config::MIN_SPEED / speed;
        }
        if speed > config::MAX_SPEED {
            movement_vector *= config::MAX_SPEED / speed;
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
