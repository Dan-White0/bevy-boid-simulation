use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use crate::configuration::config;

use super::Boid;

pub fn spawn_boids_3d(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..config::NUMBER_OF_BOIDS {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();
        let random_z = random::<f32>() * window.height();
        let random_direction_theta = random::<f32>() * std::f32::consts::TAU;
        let random_direction_phi = random::<f32>() * std::f32::consts::PI;

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(Cone {
                        radius: config::BOID_WIDTH / 2.,
                        height: config::BOID_HEIGHT,
                    })
                    .into(),
                material: materials.add(ColorMaterial::from(Color::srgb(52., 216., 235.))),
                transform: Transform::from_xyz(random_x, random_y, random_z).with_rotation(
                    Quat::from_rotation_arc(
                        Vec3::Y,
                        Vec3::new(
                            -random_direction_theta.sin(),
                            random_direction_theta.cos(),
                            random_direction_phi.sin(),
                        )
                        .normalize(),
                    ),
                ),
                ..default()
            },
            Boid {
                velocity: Vec3::new(
                    -random_direction_theta.sin(),
                    random_direction_theta.cos(),
                    random_direction_phi.sin(),
                ),
            },
        ));
    }
}
