use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera3dBundle {
        camera: Camera::default(),
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 10.0),
        ..default()
    });
}
