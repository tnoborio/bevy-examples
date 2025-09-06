use bevy::input::keyboard::Key::Character;
use bevy::{input::keyboard::KeyboardInput, prelude::*};
use bevy_ascii_terminal::{Terminal, TerminalBorder, TerminalCamera, TerminalPlugins, color};
use eff_wordlist::*;

const WIDTH: usize = 40;
const HEIGHT: usize = 7;
const GAME_SECONDS: f32 = 30.0;

#[derive(Resource)]
struct State {
    target: String,
    input: String,
    score: u32,
    time_left: f32,
    running: bool,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (tick_timer, input, draw).run_if(|state: Res<State>| state.running),
        )
        .add_systems(
            Update,
            (input_pause, draw_pause).run_if(|state: Res<State>| !state.running),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Terminal::new([WIDTH, HEIGHT]),
        TerminalBorder::single_line(),
    ));
    commands.spawn(TerminalCamera::new());

    commands.insert_resource(State {
        target: large::random_word().to_string(),
        input: String::new(),
        score: 0,
        time_left: GAME_SECONDS,
        running: true,
    });
}

fn input(mut key_events: EventReader<KeyboardInput>, mut game: ResMut<State>) {
    for key in key_events.read() {
        if key.state != bevy::input::ButtonState::Pressed {
            continue;
        }

        match key.key_code {
            KeyCode::Backspace => {
                game.input.pop();
            }
            KeyCode::Enter => {
                if game.input == game.target {
                    next_word(&mut game);
                }
            }
            _ => {
                if let Character(ch) = &key.logical_key {
                    let ch = ch.chars().next().unwrap();
                    if ch.is_ascii_alphanumeric() || ch == ' ' || ch == '-' {
                        game.input.push(ch.to_ascii_lowercase());
                    }
                }
            }
        }
    }

    if game.input == game.target {
        next_word(&mut game);
    }
}

fn next_word(game: &mut State) {
    game.score += 1;
    game.input.clear();
    game.target = eff_wordlist::large::random_word().to_string();
}

fn start_word(game: &mut State) {
    game.score = 0;
    game.time_left = GAME_SECONDS;
    game.input.clear();
    game.target = eff_wordlist::large::random_word().to_string();
    game.running = true;
}

fn tick_timer(time: Res<Time>, mut game: ResMut<State>) {
    if game.running {
        game.time_left -= time.delta().as_secs_f32();
        if game.time_left <= 0.0 {
            game.time_left = 0.0;
            game.running = false;
        }
    }
}

fn draw(mut q_term: Query<&mut Terminal>, game: Res<State>) {
    let mut term = q_term.single_mut().unwrap();
    term.clear();

    term.put_string([1, 1], format!("SCORE {:03}", game.score));
    term.put_string(
        [WIDTH as i32 - 8, 1],
        format!("TIME {:02}", game.time_left.ceil() as i32),
    );

    term.put_string([1, 3], "WORD:");
    term.put_string([7, 3], game.target.as_str());

    term.put_string([1, 5], "TYPE:");
    for (i, ch) in game.input.chars().enumerate() {
        let x = 7 + i as i32;
        let is_ok = game.target.chars().nth(i).map(|t| t == ch).unwrap_or(false);
        let col = if is_ok { color::GREEN } else { color::RED };

        term.put_char([x, 1], ch).fg(col);
    }
}

fn input_pause(mut key_events: EventReader<KeyboardInput>, mut game: ResMut<State>) {
    for key in key_events.read() {
        if key.state == bevy::input::ButtonState::Released && key.key_code == KeyCode::KeyR {
            start_word(&mut game);
        }
    }
}

fn draw_pause(mut q_term: Query<&mut Terminal>, game: Res<State>) {
    let mut term = q_term.single_mut().unwrap();
    term.clear();

    term.put_string([1, 1], format!("SCORE {:03}", game.score));

    let msg = "*** TIME UP ***".to_string();
    let hint = "Press R to restart";
    term.put_string([(WIDTH as i32 - msg.len() as i32) / 2, 2], msg);
    term.put_string([(WIDTH as i32 - hint.len() as i32) / 2, 4], hint);
}
