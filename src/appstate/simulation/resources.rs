use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct BoundingBox(pub Vec3);

impl Default for BoundingBox {
    fn default() -> Self {
        Self(Vec3::ZERO)
    }
}
