use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn set_2d_simulation_view(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let mut camera = camera_query.get_single_mut().unwrap();

    camera.translation = Vec3::new(
        window.width() / 2.0,
        window.height() / 2.0,
        window.width() * 0.8,
    );
    camera.look_at(
        Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.),
        Vec3::Y,
    );
}
