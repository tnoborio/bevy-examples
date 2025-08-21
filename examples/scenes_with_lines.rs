use bevy::prelude::*;
use rand::prelude::*;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
enum SceneState {
    #[default]
    A,
    B,
}

#[derive(Component)]
struct ARoot;

#[derive(Component)]
struct BRoot;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .init_state::<SceneState>()
        .add_systems(OnEnter(SceneState::A), enter_a)
        .add_systems(OnExit(SceneState::A), cleanup_a)
        .add_systems(OnEnter(SceneState::B), enter_b)
        .add_systems(OnExit(SceneState::B), cleanup_b)
        .add_systems(Update, toggle_scene_on_space)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn enter_a(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((Transform::default(), ARoot)).with_children(|p| {
        let mut r = rand::rng();
        for _n in 1..25 {
            let circle = meshes.add(Circle::new(80.0));
            p.spawn((
                Mesh2d(circle),
                MeshMaterial2d(materials.add(Color::srgba(
                    r.random_range(0.0..1.0),
                    r.random_range(0.0..1.0),
                    r.random_range(0.0..1.0),
                    0.5,
                ))),
                Transform::from_xyz(
                    r.random_range(-240.0..240.0),
                    r.random_range(-240.0..240.0),
                    r.random_range(-240.0..240.0),
                ),
            ));
        }
    });
}

fn cleanup_a(
    mut commands: Commands, q: Query<Entity, With<ARoot>>
) {
    for root in &q {
        commands.entity(root).despawn();
    }
}

fn enter_b(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
 ) {
    commands.spawn((Transform::default(), BRoot)).with_children(|p| {
        let mut r = rand::rng();
        for _n in 1..25 {
            let line = meshes.add(Rectangle::new(300.0, 4.0));
            p.spawn((
                Mesh2d(line),
                MeshMaterial2d(materials.add(Color::srgba(
                    r.random_range(0.0..1.0),
                    r.random_range(0.0..1.0),
                    r.random_range(0.0..1.0),
                    0.5,
                ))),
                Transform::from_xyz(
                    r.random_range(-240.0..240.0),
                    r.random_range(-240.0..240.0),
                    r.random_range(-240.0..240.0),
                ).with_rotation(Quat::from_rotation_z((r.random_range(0.0f32..180.0f32)).to_radians()))
            ));
        }
    });
}

fn cleanup_b(
    mut commands: Commands, q: Query<Entity, With<BRoot>>
) {
    for root in &q {
        commands.entity(root).despawn();
    }
}

fn toggle_scene_on_space(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<SceneState>>,
    mut next: ResMut<NextState<SceneState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        match *state.get() {
            SceneState::A => next.set(SceneState::B),
            SceneState::B => next.set(SceneState::A),
        }
    }
}
