use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::utils::HashMap;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

mod camera;

use camera::spawn_camera;

const NUMBER_OF_BOIDS: usize = 200;
const BOID_HEIGHT: f32 = 30.;
const BOID_WIDTH: f32 = 20.;
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
        .add_systems(Startup, (spawn_camera, spawn_boids))
        .add_systems(FixedUpdate, move_boids)
        .run();
}

#[derive(Component)]
pub struct Boid {
    vx: f32,
    vy: f32,
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
                        .add(Triangle2d::new(
                            Vec2::new(-BOID_WIDTH / 2., -BOID_HEIGHT / 2.),
                            Vec2::new(0., BOID_HEIGHT / 2.),
                            Vec2::new(BOID_WIDTH / 2., -BOID_HEIGHT / 2.),
                        ))
                        .into(),
                    material: materials.add(ColorMaterial::from(Color::srgb(52., 216., 235.))),
                    transform: Transform::from_xyz(random_x, random_y, 0.)
                        .with_rotation(Quat::from_rotation_z(random_rotation)),
                    ..default()
                },
                Boid {
                    vx: -random_rotation.sin(),
                    vy: random_rotation.cos(),
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
) {
    let window = window_query.get_single().unwrap();
    let mut movement_vectors: HashMap<Entity, Vec3> = HashMap::new();

    for (boid_a_transform, boid_a_entity, boid_a) in boid_query.iter() {
        let mut xpos_avg = 0.;
        let mut ypos_avg = 0.;
        let mut xvel_avg = 0.;
        let mut yvel_avg = 0.;
        let mut neighboring_boids = 0.;
        let mut close_dx = 0.;
        let mut close_dy = 0.;

        for (boid_b_transform, boid_b_entity, boid_b) in boid_query.iter() {
            if boid_a_entity == boid_b_entity {
                continue;
            }

            let dx = boid_a_transform.translation.x - boid_b_transform.translation.x;
            let dy = boid_a_transform.translation.y - boid_b_transform.translation.y;

            if dx.abs() < VISUAL_RANGE && dy.abs() < VISUAL_RANGE {
                let squared_distance = dx * dx + dy * dy;

                if squared_distance < PROTECTED_RANGE_SQUARED {
                    close_dx += boid_a_transform.translation.x - boid_b_transform.translation.x;
                    close_dy += boid_a_transform.translation.y - boid_b_transform.translation.y;
                } else if squared_distance < VISUAL_RANGE_SQUARED {
                    xpos_avg += boid_b_transform.translation.x;
                    ypos_avg += boid_b_transform.translation.y;
                    xvel_avg += boid_b.vx;
                    yvel_avg += boid_b.vy;

                    neighboring_boids += 1.;
                }
            }
        }

        let mut new_vx = boid_a.vx;
        let mut new_vy = boid_a.vy;

        if neighboring_boids > 0. {
            xpos_avg = xpos_avg / neighboring_boids;
            ypos_avg = ypos_avg / neighboring_boids;
            xvel_avg = xvel_avg / neighboring_boids;
            yvel_avg = yvel_avg / neighboring_boids;

            new_vx += boid_a.vx
                + (xpos_avg - boid_a_transform.translation.x) * CENTERING_FACTOR
                + (xvel_avg - boid_a.vx) * MATCHING_FACTOR;

            new_vy += boid_a.vy
                + (ypos_avg - boid_a_transform.translation.y) * CENTERING_FACTOR
                + (yvel_avg - boid_a.vy) * MATCHING_FACTOR;
        }

        new_vx += close_dx * AVOID_FACTOR;
        new_vy += close_dy * AVOID_FACTOR;

        if boid_a_transform.translation.x < LEFT_MARGIN {
            new_vx += TURN_FACTOR
        }
        if boid_a_transform.translation.x > window.width() - RIGHT_MARGIN {
            new_vx -= TURN_FACTOR
        }
        if boid_a_transform.translation.y < TOP_MARGIN {
            new_vy += TURN_FACTOR
        }
        if boid_a_transform.translation.y > window.height() - BOTTOM_MARGIN {
            new_vy -= TURN_FACTOR
        }

        let mut movement_vector = Vec3::new(new_vx, new_vy, 0.);
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
        boid.vx = movement_vector.x;
        boid.vy = movement_vector.y;

        boid_transform.rotation = Quat::from_rotation_arc(Vec3::Y, movement_vector.normalize());
    }
}
