use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use crate::configuration::config;

use super::Boid;

pub fn spawn_boids_2d(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..config::NUMBER_OF_BOIDS {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();
        let random_rotation = random::<f32>() * std::f32::consts::TAU;

        commands
            .spawn((
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Cone {
                            radius: config::BOID_WIDTH / 2.,
                            height: config::BOID_HEIGHT,
                        })
                        .into(),
                    material: materials.add(ColorMaterial::from(Color::srgb(52., 216., 235.))),
                    transform: Transform::from_xyz(random_x, random_y, 0.)
                        .with_rotation(Quat::from_rotation_z(random_rotation)),
                    ..default()
                },
                Boid {
                    velocity: Vec3::new(-random_rotation.sin(), random_rotation.cos(), 0.),
                },
            ))
            .with_children(|parent| {
                let view_cone = Mesh2dHandle(meshes.add(CircularSector::new(
                    config::VISUAL_RANGE,
                    config::BOID_VIEW_ANGLE_RAD,
                )));

                parent.spawn(MaterialMesh2dBundle {
                    mesh: view_cone,
                    material: materials.add(ColorMaterial::from(Color::srgb(0., 255., 0.))),
                    transform: Transform::from_xyz(0., 0., -1.),
                    visibility: match config::SHOW_VIEW_CONE {
                        true => Visibility::Visible,
                        false => Visibility::Hidden,
                    },
                    ..default()
                });

                let avoid_cone = Mesh2dHandle(meshes.add(CircularSector::new(
                    config::PROTECTED_RANGE,
                    config::BOID_VIEW_ANGLE_RAD,
                )));
                parent.spawn(MaterialMesh2dBundle {
                    mesh: avoid_cone,
                    material: materials.add(ColorMaterial::from(Color::srgb(255., 0., 0.))),
                    transform: Transform::from_xyz(0., 0., -0.5),
                    visibility: match config::SHOW_VIEW_CONE {
                        true => Visibility::Visible,
                        false => Visibility::Hidden,
                    },
                    ..default()
                });
            });
    }
}
