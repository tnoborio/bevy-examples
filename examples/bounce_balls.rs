use std::time::Duration;

use bevy::math::primitives::{Circle, Rectangle};
use bevy::prelude::*;
use bevy::render::mesh::Mesh;
use bevy::time::common_conditions::on_timer;
use bevy_rapier2d::prelude::*;

use rand::Rng;

const GROUND_W: f32 = 1000.0;
const GROUND_H: f32 = 10.0;
const GROUND_Y: f32 = -300.0;
const GROUND_COLOR: Color = Color::srgb(1.0, 0.7, 0.4);

const BALL_R: f32 = 10.0;
const BALL_START_Y: f32 = 400.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_systems(Startup, (camera, spawn_world))
        .add_systems(
            Update,
            spawn_ball.run_if(on_timer(Duration::from_millis(40))),
        )
        .run();
}

fn camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let ground_mesh = Mesh::from(Rectangle::new(GROUND_W, GROUND_H));
    commands.spawn((
        Mesh2d(meshes.add(ground_mesh).into()),
        MeshMaterial2d(materials.add(GROUND_COLOR)),
        Transform::from_xyz(0.0, GROUND_Y, -0.1),
        RigidBody::Fixed,
        Collider::cuboid(GROUND_W * 0.5, GROUND_H * 0.5),
        Friction::coefficient(0.6),
    ));
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let ball_mesh = Mesh::from(Circle::new(BALL_R));
    let mut r = rand::rng();
    let x = r.random_range(-20.0..20.0);
    let color = Color::srgb(
        r.random::<f32>() * 0.3 + 0.7,
        r.random::<f32>() * 0.3 + 0.7,
        1.0,
    );

    commands.spawn((
        Mesh2d(meshes.add(ball_mesh).into()),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(x, BALL_START_Y, 0.1),
        RigidBody::Dynamic,
        Collider::ball(BALL_R),
        Friction {
            coefficient: 0.9,
            combine_rule: CoefficientCombineRule::Min,
        },
        Restitution {
            coefficient: 0.4,
            combine_rule: CoefficientCombineRule::Max,
        },
        Damping {
            linear_damping: 0.01,
            angular_damping: 0.05,
        },
    ));
}
