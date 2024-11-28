use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct RunSimulation(pub bool);

impl Default for RunSimulation {
    fn default() -> Self {
        Self(true)
    }
}
