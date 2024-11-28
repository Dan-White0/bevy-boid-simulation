use bevy::prelude::*;

use super::Boid;

pub fn despawn_boids(mut commands: Commands, boid_query: Query<Entity, With<Boid>>) {
    for boid_entity in boid_query.iter() {
        commands.entity(boid_entity).despawn_recursive();
    }
}
