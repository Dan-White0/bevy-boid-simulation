use super::BoundingBox;
use bevy::window::PrimaryWindow;

use bevy::prelude::*;

pub fn set_bounding_box_3d(
    mut bounding_box: ResMut<BoundingBox>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    bounding_box.0 = Vec3::new(window.width(), window.width(), window.width());
}
