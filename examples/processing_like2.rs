use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use rand::Rng;
use std::cell::RefCell;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex};

const CANVAS_W: f32 = 400.0;
const CANVAS_H: f32 = 400.0;

fn canvas_to_world(p: Vec2) -> Vec2 {
    Vec2::new(p.x - CANVAS_W * 0.5, CANVAS_H * 0.5 - p.y)
}

#[derive(Clone)]
enum ProcessingCommand {
    Line {
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        thickness: f32,
        color: Color,
    },
    Rect {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: Color,
    },
    Ellipse {
        cx: f32,
        cy: f32,
        w: f32,
        h: f32,
        color: Color,
    },
    Triangle {
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        color: Color,
    },
}

thread_local! {
    static TX: RefCell<Option<Sender<ProcessingCommand>>> = RefCell::new(None);
}
fn install_tx(sender: Sender<ProcessingCommand>) {
    TX.with(|c| *c.borrow_mut() = Some(sender));
}
fn send(cmd: ProcessingCommand) {
    TX.with(|c| {
        if let Some(tx) = &*c.borrow() {
            let _ = tx.send(cmd);
        }
    });
}

pub fn line(x1: f32, y1: f32, x2: f32, y2: f32, color: Color) {
    send(ProcessingCommand::Line {
        x1,
        y1,
        x2,
        y2,
        thickness: 4.0,
        color: color,
    });
}
pub fn rect(x: f32, y: f32, w: f32, h: f32, color: Color) {
    send(ProcessingCommand::Rect {
        x,
        y,
        w,
        h,
        color: color,
    });
}
pub fn ellipse(cx: f32, cy: f32, w: f32, h: f32, color: Color) {
    send(ProcessingCommand::Ellipse {
        cx,
        cy,
        w,
        h,
        color: color,
    });
}
pub fn triangle(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, color: Color) {
    send(ProcessingCommand::Triangle {
        x1,
        y1,
        x2,
        y2,
        x3,
        y3,
        color: color,
    });
}

#[derive(Resource, Clone)]
struct DrawRx(Arc<Mutex<Receiver<ProcessingCommand>>>);

fn rasterize_and_spawn(
    mut commands: Commands,
    rx: Res<DrawRx>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut drained = Vec::new();
    {
        let guard = rx.0.lock().unwrap();
        while let Ok(cmd) = guard.try_recv() {
            drained.push(cmd);
        }
    }
    for cmd in drained {
        match cmd {
            ProcessingCommand::Line {
                x1,
                y1,
                x2,
                y2,
                thickness,
                color,
            } => {
                let a = canvas_to_world(Vec2::new(x1, y1));
                let b = canvas_to_world(Vec2::new(x2, y2));
                let delta = b - a;
                let len = delta.length();
                let angle = delta.to_angle();

                let line_mesh = meshes.add(Rectangle {
                    half_size: Vec2::new(len * 0.5, thickness * 0.5),
                });
                commands.spawn((
                    Mesh2d(line_mesh),
                    MeshMaterial2d(materials.add(color)),
                    Transform {
                        translation: Vec3::new((a.x + b.x) * 0.5, (a.y + b.y) * 0.5, 0.0),
                        rotation: Quat::from_rotation_z(angle),
                        ..default()
                    },
                ));
            }
            ProcessingCommand::Rect { x, y, w, h, color } => {
                let center = canvas_to_world(Vec2::new(x + w * 0.5, y + h * 0.5));
                let rect_mesh = meshes.add(Rectangle {
                    half_size: Vec2::new(w * 0.5, h * 0.5),
                });
                commands.spawn((
                    Mesh2d(rect_mesh),
                    MeshMaterial2d(materials.add(color)),
                    Transform::from_xyz(center.x, center.y, 0.0),
                ));
            }
            ProcessingCommand::Ellipse {
                cx,
                cy,
                w,
                h,
                color,
            } => {
                let center = canvas_to_world(Vec2::new(cx, cy));
                let circle_mesh = meshes.add(Circle { radius: 0.5 });
                commands.spawn((
                    Mesh2d(circle_mesh),
                    MeshMaterial2d(materials.add(color)),
                    Transform {
                        translation: Vec3::new(center.x, center.y, 0.0),
                        scale: Vec3::new(w, h, 1.0),
                        ..default()
                    },
                ));
            }
            ProcessingCommand::Triangle {
                x1,
                y1,
                x2,
                y2,
                x3,
                y3,
                color,
            } => {
                let a = canvas_to_world(Vec2::new(x1, y1));
                let b = canvas_to_world(Vec2::new(x2, y2));
                let c = canvas_to_world(Vec2::new(x3, y3));
                let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, default());
                mesh.insert_attribute(
                    Mesh::ATTRIBUTE_POSITION,
                    vec![[a.x, a.y, 0.0], [b.x, b.y, 0.0], [c.x, c.y, 0.0]],
                );
                mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 3]);
                mesh.insert_attribute(
                    Mesh::ATTRIBUTE_UV_0,
                    vec![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]],
                );
                mesh.insert_indices(Indices::U32(vec![0, 1, 2]));
                let tri = meshes.add(mesh);

                commands.spawn((
                    Mesh2d(tri),
                    MeshMaterial2d(materials.add(color)),
                    Transform::default(),
                ));
            }
        }
    }
}

fn random_color() -> Color {
    let mut r = rand::rng();
    Color::srgb(r.random(), r.random(), r.random())
}

fn user_sketch() {
    ellipse(200.0, 200.0, 400.0, 400.0, random_color());
    line(0.0, 0.0, 200.0, 200.0, Color::linear_rgb(1.0, 0.0, 0.0));
    rect(100.0, 100.0, 150.0, 100.0, Color::linear_rgb(1.0, 1.0, 0.0));
    ellipse(300.0, 250.0, 150.0, 250.0, Color::linear_rgb(1.0, 0.0, 1.0));
    triangle(100.0, 250.0, 50.0, 350.0, 300.0, 350.0, Color::linear_rgb(1.0, 0.5, 0.0));

    for _n in 0..11 {
        let mut r = rand::rng();
        let x1 = r.random_range(0.0..CANVAS_W);
        let y1 = r.random_range(0.0..CANVAS_H);
        let x2 = r.random_range(0.0..CANVAS_W);
        let y2 = r.random_range(0.0..CANVAS_H);
        line(x1, y1, x2, y2, random_color());
    };
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (CANVAS_W, CANVAS_H).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(
            Startup,
            (
                setup_camera,
                setup_pipeline,
                run_sketch,
                rasterize_and_spawn,
            )
                .chain(),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_pipeline(mut commands: Commands) {
    let (tx, rx) = channel::<ProcessingCommand>();
    install_tx(tx);
    commands.insert_resource(DrawRx(Arc::new(Mutex::new(rx))));
}

fn run_sketch() {
    user_sketch();
}
