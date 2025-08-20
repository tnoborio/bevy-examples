use bevy::prelude::*;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
enum SceneState {
    #[default]
    Title,
    Play,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<SceneState>()
        .add_systems(OnEnter(SceneState::Title), enter_title)
        .add_systems(OnEnter(SceneState::Play), enter_play)
        .add_systems(Update, toggle_scene_on_space)
        .run();
}

fn enter_title(mut clear: ResMut<ClearColor>) {
    clear.0 = Color::srgb(0.5, 0.0, 0.0)
}

fn enter_play(mut clear: ResMut<ClearColor>) {
    clear.0 = Color::srgb(0.0, 0.0, 0.5)
}

fn toggle_scene_on_space(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<SceneState>>,
    mut next: ResMut<NextState<SceneState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        match *state.get() {
            SceneState::Title => next.set(SceneState::Play),
            SceneState::Play => next.set(SceneState::Title),
        }
    }
}