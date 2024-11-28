use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use super::super::Boid;

const NUMBER_OF_BOIDS: usize = 200;
const BOID_SIZE_SCALE: f32 = 5.;
const BOID_HEIGHT: f32 = 3. * BOID_SIZE_SCALE;
const BOID_WIDTH: f32 = 2. * BOID_SIZE_SCALE;
const BOID_VIEW_ANGLE_RAD: f32 = std::f32::consts::PI * 3. / 4.;
const SHOW_VIEW_CONE: bool = false;
const VISUAL_RANGE: f32 = 80.;
const PROTECTED_RANGE: f32 = 20.;

pub fn spawn_boids_2d(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_BOIDS {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();
        let random_rotation = random::<f32>() * std::f32::consts::TAU;

        commands
            .spawn((
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Cone {
                            radius: BOID_WIDTH / 2.,
                            height: BOID_HEIGHT,
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
                let view_cone = Mesh2dHandle(
                    meshes.add(CircularSector::new(VISUAL_RANGE, BOID_VIEW_ANGLE_RAD)),
                );

                parent.spawn(MaterialMesh2dBundle {
                    mesh: view_cone,
                    material: materials.add(ColorMaterial::from(Color::srgb(0., 255., 0.))),
                    transform: Transform::from_xyz(0., 0., -1.),
                    visibility: match SHOW_VIEW_CONE {
                        true => Visibility::Visible,
                        false => Visibility::Hidden,
                    },
                    ..default()
                });

                let avoid_cone = Mesh2dHandle(
                    meshes.add(CircularSector::new(PROTECTED_RANGE, BOID_VIEW_ANGLE_RAD)),
                );
                parent.spawn(MaterialMesh2dBundle {
                    mesh: avoid_cone,
                    material: materials.add(ColorMaterial::from(Color::srgb(255., 0., 0.))),
                    transform: Transform::from_xyz(0., 0., -0.5),
                    visibility: match SHOW_VIEW_CONE {
                        true => Visibility::Visible,
                        false => Visibility::Hidden,
                    },
                    ..default()
                });
            });
    }
}
