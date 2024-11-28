use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::utils::HashMap;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

mod view;

use view::spawn_camera;

const NUMBER_OF_BOIDS: usize = 200;
const BOID_SIZE_SCALE: f32 = 5.;
const BOID_HEIGHT: f32 = 3. * BOID_SIZE_SCALE;
const BOID_WIDTH: f32 = 2. * BOID_SIZE_SCALE;
const BOID_VIEW_ANGLE_RAD: f32 = std::f32::consts::PI * 3. / 4.;
const SHOW_VIEW_CONE: bool = false;

const LEFT_MARGIN: f32 = 75.;
const RIGHT_MARGIN: f32 = 75.;
const TOP_MARGIN: f32 = 75.;
const BOTTOM_MARGIN: f32 = 75.;

const TURN_FACTOR: f32 = 0.5;
const VISUAL_RANGE: f32 = 80.;
const VISUAL_RANGE_SQUARED: f32 = VISUAL_RANGE * VISUAL_RANGE;
const PROTECTED_RANGE: f32 = 20.;
const PROTECTED_RANGE_SQUARED: f32 = PROTECTED_RANGE * PROTECTED_RANGE;
const CENTERING_FACTOR: f32 = 0.005;
const AVOID_FACTOR: f32 = 0.075;
const MATCHING_FACTOR: f32 = 0.2;
const MAX_SPEED: f32 = 6.;
const MIN_SPEED: f32 = 3.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<RunSimulation>()
        .add_systems(Startup, (spawn_camera, spawn_boids))
        .add_systems(Update, start_stop)
        .add_systems(FixedUpdate, move_boids)
        .run();
}

#[derive(Component)]
pub struct Boid {
    velocity: Vec3,
}

fn spawn_boids(
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

fn move_boids(
    mut boid_query: Query<(&mut Transform, Entity, &mut Boid)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    run_simulation: ResMut<RunSimulation>,
) {
    if !run_simulation.0 {
        return;
    }

    let window = window_query.get_single().unwrap();
    let mut movement_vectors: HashMap<Entity, Vec3> = HashMap::new();

    for (boid_a_transform, boid_a_entity, boid_a) in boid_query.iter() {
        let mut pos_avg = Vec3::ZERO;
        let mut vel_avg = Vec3::ZERO;
        let mut close_distance = Vec3::ZERO;

        let mut neighboring_boids = 0.;

        for (boid_b_transform, boid_b_entity, boid_b) in boid_query.iter() {
            if boid_a_entity == boid_b_entity {
                continue;
            }

            let vector_between = boid_a_transform.translation - boid_b_transform.translation;

            if vector_between.x.abs() < VISUAL_RANGE && vector_between.y.abs() < VISUAL_RANGE {
                let squared_distance = vector_between.distance_squared(Vec3::ZERO);

                if squared_distance < PROTECTED_RANGE_SQUARED {
                    close_distance += vector_between;
                } else if squared_distance < VISUAL_RANGE_SQUARED {
                    pos_avg += boid_b_transform.translation;
                    vel_avg += boid_b.velocity;

                    neighboring_boids += 1.;
                }
            }
        }

        let mut new_velocity = boid_a.velocity;

        if neighboring_boids > 0. {
            pos_avg /= neighboring_boids;
            vel_avg /= neighboring_boids;

            new_velocity += new_velocity
                + (pos_avg - boid_a_transform.translation) * CENTERING_FACTOR
                + (vel_avg - new_velocity) * MATCHING_FACTOR;
        }

        new_velocity += close_distance * AVOID_FACTOR;

        if boid_a_transform.translation.x < LEFT_MARGIN {
            new_velocity += Vec3::new(TURN_FACTOR, 0., 0.);
        }
        if boid_a_transform.translation.x > window.width() - RIGHT_MARGIN {
            new_velocity -= Vec3::new(TURN_FACTOR, 0., 0.);
        }
        if boid_a_transform.translation.y < TOP_MARGIN {
            new_velocity += Vec3::new(0., TURN_FACTOR, 0.);
        }
        if boid_a_transform.translation.y > window.height() - BOTTOM_MARGIN {
            new_velocity -= Vec3::new(0., TURN_FACTOR, 0.);
        }

        let mut movement_vector = new_velocity;
        let speed = movement_vector.length();

        if speed < MIN_SPEED {
            movement_vector *= MIN_SPEED / speed;
        }
        if speed > MAX_SPEED {
            movement_vector *= MAX_SPEED / speed;
        }

        movement_vectors.insert(boid_a_entity, movement_vector);
    }

    for (mut boid_transform, boid_entity, mut boid) in boid_query.iter_mut() {
        let movement_vector = *movement_vectors
            .get(&boid_entity)
            .expect("All boids should have a movement vector in the map");

        boid_transform.translation += movement_vector;
        boid.velocity = movement_vector;

        boid_transform.rotation = Quat::from_rotation_arc(Vec3::Y, movement_vector.normalize());
    }
}

#[derive(Resource, Debug, Default)]
pub struct RunSimulation(bool);

pub fn start_stop(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut run_simulation: ResMut<RunSimulation>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        run_simulation.0 = !run_simulation.0;
    }
}
