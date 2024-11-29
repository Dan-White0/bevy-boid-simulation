use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn set_main_menu_view(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // let mut camera_transform = camera_query.get_single_mut().unwrap();
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        *camera_transform = Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 10.0);
    }
}
