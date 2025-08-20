use bevy::prelude::*;

fn change_clear_color(keys: Res<ButtonInput<KeyCode>>, mut clear_color: ResMut<ClearColor>) {
    if keys.just_pressed(KeyCode::Space) {
        clear_color.0 = Color::WHITE;
    } else if keys.just_pressed(KeyCode::Enter) {
        clear_color.0 = Color::BLACK;
    }
}

fn setup_clear_color(mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = Color::BLACK;
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_clear_color)
        .add_systems(Update, change_clear_color)
        .run();
}
