#![allow(clippy::needless_pass_by_value)]

use bevy::{
    prelude::*,
    sprite::{collide_aabb::{collide, Collision}, MaterialMesh2dBundle},
};
use rand::{thread_rng, Rng};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_startup_system(add_squares)
        // .add_system(report_squares)
        .add_system(movement_system)
        .add_system(wall_collision_system)
        .add_system(collision_system)
        .run();
}

#[derive(Component)]
struct Square {
    velocity: Vec2,
}

#[derive(Component)]
struct Size(u32);

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Default)]
struct CollisionEvent;

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn add_squares(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = thread_rng();
    for i in 1..500u32 {
        commands
            .spawn_bundle(MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Quad::new(Vec2 { x: 10., y: 10. })))
                    .into(),
                transform: Transform::from_translation(Vec3::new(-500. + (i * 5) as f32, 0., 0.)),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                ..default()
            })
            .insert(Square {
                velocity: Vec2::new(rng.gen_range(1..100) as f32, rng.gen_range(1..100) as f32),
            })
            .insert(Size(10))
            .insert(Collider);
    }

    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Quad::new(Vec2 { x: 20., y: 20. })))
            .into(),
        transform: Transform::from_translation(Vec3::new(-100., 50., 1.)),
        material: materials.add(ColorMaterial::from(Color::YELLOW)),
        ..default()
    });
}

fn report_squares(time: Res<Time>, query: Query<&Size, &Square>) {
    let count = query.iter().count();
    query
        .iter()
        .for_each(|q| println!("Size {}, count {}", q.0, count));
}

fn movement_system(time: Res<Time>, mut query: Query<(&Square, &mut Transform)>) {
    for (square, mut transform) in &mut query {
        transform.translation.x += square.velocity.x * time.delta_seconds();
        transform.translation.y += square.velocity.y * time.delta_seconds();
    }
}

fn wall_collision_system(
    windows: Res<Windows>,
    mut query: Query<(&mut Square, &Size, &Transform)>,
) {
    let window = windows.primary();
    let half_width = window.width() as f32 * 0.5;
    let half_height = window.height() as f32 * 0.5;

    for (mut square, size, transform) in &mut query {
        let half_size = (size.0 / 1) as f32;
        let x_vel = square.velocity.x;
        let y_vel = square.velocity.y;
        let x_pos = transform.translation.x;
        let y_pos = transform.translation.y;

        if y_pos + half_size > half_height || y_pos - half_size < -half_height {
            square.velocity.y = -y_vel;
        }

        if x_pos + half_size > half_width || x_pos - half_size < -half_width {
            square.velocity.x = -x_vel;
        }
    }
}

fn collision_system(
    mut query: Query<(&mut Square, &Size, &Transform)>,
    collider: Query<(&Collider, &Size, &Transform)>,
) {
    for (mut square, size, transform) in &mut query {
        let half_size = (size.0 / 4) as f32;

        for (_other_square, other_size, other_transform) in &collider {
            let collision = collide(
                transform.translation,
                Vec2 {
                    x: half_size,
                    y: half_size,
                },
                other_transform.translation,
                Vec2 {
                    x: half_size,
                    y: half_size,
                },
            );

            if let Some(collision) = collision {
                // reflect the ball when it collides
                let mut reflect_x = false;
                let mut reflect_y = false;

                // only reflect if the ball's velocity is going in the opposite direction of the
                // collision
                match collision {
                    Collision::Left => reflect_x = square.velocity.x > 0.0,
                    Collision::Right => reflect_x = square.velocity.x < 0.0,
                    Collision::Top => reflect_y = square.velocity.y < 0.0,
                    Collision::Bottom => reflect_y = square.velocity.y > 0.0,
                    Collision::Inside => { /* do nothing */ }
                }

                // reflect velocity on the x-axis if we hit something on the x-axis
                if reflect_x {
                    square.velocity.x = -square.velocity.x;
                }

                // reflect velocity on the y-axis if we hit something on the y-axis
                if reflect_y {
                    square.velocity.y = -square.velocity.y;
                }
            }
        }
    }
}