use bevy::prelude::*;
use rand::Rng;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;

#[derive(Resource)]
struct FontHandle(Handle<Font>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup_font, setup_texts).chain())
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::FixedVertical {
                viewport_height: HEIGHT as f32,
            },
            scale: 1.0,
            ..OrthographicProjection::default_2d()
        }),
    ));
}

fn setup_font(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(FontHandle(assets.load("LinestriderRegular-PjJd 2.ttf")));
}

fn setup_texts(mut commands: Commands, font: Res<FontHandle>) {
    let mut rng = rand::rng();

    for _n in 0..20 {
        let x = rng.random_range((-(WIDTH as i32) / 2)..((WIDTH as i32) / 2)) as f32;
        let y = rng.random_range((-(HEIGHT as i32) / 2)..((HEIGHT as i32) / 2)) as f32;
        let color = Color::linear_rgba(
            rng.random_range(0.0..1.0),
            rng.random_range(0.0..1.0),
            rng.random_range(0.0..1.0),
            0.5,
        );

        commands
            .spawn(Node {
                position_type: PositionType::Absolute,
                ..default()
            })
            .with_children(|p| {
                p.spawn((
                    Text2d::new("text"),
                    TextFont {
                        font: font.0.clone(),
                        font_size: 80.0,
                        ..default()
                    },
                    TextColor(color),
                    Transform::from_translation(Vec3::new(x, y, 0.0)),
                ));
            });
    }
}
