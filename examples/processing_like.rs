use bevy::math::primitives::{Circle, Rectangle};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (400.0, 400.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_camera, draw))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

// Canvas座標(左上0,0)→World座標変換
fn canvas_to_world(p: Vec2) -> Vec2 {
    Vec2::new(p.x - 400.0 * 0.5, 400.0 * 0.5 - p.y)
}

fn draw(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // ===== 線 (line) =====
    let a_canvas = Vec2::new(50.0, 50.0);
    let b_canvas = Vec2::new(350.0, 350.0);
    let a = canvas_to_world(a_canvas);
    let b = canvas_to_world(b_canvas);
    let thickness = 4.0;

    let delta = b - a;
    let len = delta.length();
    let angle = delta.to_angle();

    let line_mesh = meshes.add(Rectangle {
        half_size: Vec2::new(len * 0.5, thickness * 0.5),
    });
    commands.spawn((
        Mesh2d(line_mesh),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform {
            translation: Vec3::new((a.x + b.x) * 0.5, (a.y + b.y) * 0.5, 0.0),
            rotation: Quat::from_rotation_z(angle),
            ..default()
        },
    ));

    // ===== 四角形 (rect) =====
    let rect_pos = Vec2::new(100.0, 120.0);
    let rect_size = Vec2::new(120.0, 80.0);
    let rect_center = canvas_to_world(rect_pos + rect_size * 0.5);

    let rect_mesh = meshes.add(Rectangle {
        half_size: rect_size * 0.5,
    });
    commands.spawn((
        Mesh2d(rect_mesh),
        MeshMaterial2d(materials.add(Color::srgb_u8(0, 170, 0))),
        Transform::from_xyz(rect_center.x, rect_center.y, 0.0),
    ));

    // ===== 円 (circle) =====
    let circle_center_canvas = Vec2::new(200.0, 200.0);
    let circle_center = canvas_to_world(circle_center_canvas);
    let radius = 60.0;

    let circle_mesh = meshes.add(Circle { radius });
    commands.spawn((
        Mesh2d(circle_mesh),
        MeshMaterial2d(materials.add(Color::srgb_u8(30, 144, 255))),
        Transform::from_xyz(circle_center.x, circle_center.y, 0.0),
    ));
}
