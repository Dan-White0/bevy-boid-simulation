use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn set_3d_simulation_view(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let mut camera = camera_query.get_single_mut().unwrap();

    camera.translation = Vec3::new(
        window.width() * 2.,
        window.width() * 1.6,
        window.width() * 1.5,
    );
    camera.look_at(
        Vec3::new(
            window.width() / 2.,
            window.width() / 2. - window.width() * 0.1,
            window.width() / 2.,
        ),
        Vec3::Y,
    );
}
