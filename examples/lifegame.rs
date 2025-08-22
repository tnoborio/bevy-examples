use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_ascii_terminal::{Terminal, TerminalCamera, TerminalPlugins, Tile, ascii, color};
use rand::Rng;

const WIDTH: usize = 80;
const HEIGHT: usize = 40;
const STEP_SEC: f32 = 0.25;

#[derive(Resource)]
struct Board {
    w: usize,
    h: usize,
    cells: Vec<bool>,
    next: Vec<bool>,
    running: bool,
}

impl Board {
    fn new(w: usize, h: usize) -> Self {
        let mut rng = rand::rng();
        let mut cells = vec![false; w * h];
        for c in cells.iter_mut() {
            *c = rng.random_bool(0.25);
        }
        Self {
            w,
            h,
            next: vec![false; w * h],
            cells,
            running: true,
        }
    }

    #[inline]
    fn idx(&self, x: usize, y: usize) -> usize {
        y * self.w + x
    }

    fn neighbor_count(&self, x: usize, y: usize) -> u8 {
        let w = self.w as isize;
        let h = self.h as isize;
        let xi = x as isize;
        let yi = y as isize;
        let mut c = 0u8;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = (xi + dx + w) % w;
                let ny = (yi + dy + h) % h;
                if self.cells[self.idx(nx as usize, ny as usize)] {
                    c += 1;
                }
            }
        }
        c
    }

    fn step(&mut self) {
        for y in 0..self.h {
            for x in 0..self.w {
                let i = self.idx(x, y);
                let n = self.neighbor_count(x, y);
                self.next[i] = match (self.cells[i], n) {
                    (true, 2 | 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
            }
        }
        std::mem::swap(&mut self.cells, &mut self.next);
    }

    fn randomize(&mut self, p_alive: f64) {
        let mut rng = rand::rng();
        for c in self.cells.iter_mut() {
            *c = rng.random_bool(p_alive);
        }
    }

    fn clear(&mut self) {
        for c in self.cells.iter_mut() {
            *c = false;
        }
    }
}

#[derive(Resource)]
struct StepTimer(Timer);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .insert_resource(Board::new(WIDTH, HEIGHT))
        .insert_resource(StepTimer(Timer::from_seconds(
            STEP_SEC,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, setup)
        .add_systems(Update, (tick, input))
        .add_systems(
            Update,
            draw.run_if(on_timer(Duration::from_secs_f32(STEP_SEC))),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Terminal::new([WIDTH, HEIGHT]),));
    commands.spawn(TerminalCamera::new());
}

fn tick(time: Res<Time>, mut timer: ResMut<StepTimer>, mut board: ResMut<Board>) {
    if !board.running {
        return;
    }
    if timer.0.tick(time.delta()).just_finished() {
        board.step();
    }
}

fn input(keys: Res<ButtonInput<KeyCode>>, mut board: ResMut<Board>) {
    if keys.just_pressed(KeyCode::Space) {
        board.running = !board.running;
    }
    if keys.just_pressed(KeyCode::KeyR) {
        board.randomize(0.25);
    }
    if keys.just_pressed(KeyCode::KeyC) {
        board.clear();
    }
    if keys.just_pressed(KeyCode::KeyS) && !board.running {
        board.step();
    }
}

fn draw(mut q_term: Query<&mut Terminal>, board: Res<Board>) {
    let mut term = q_term.single_mut().unwrap();
    term.clear();
    let mut rng = rand::rng();

    for y in 0..board.h {
        for x in 0..board.w {
            if board.cells[board.idx(x, y)] {
                let index = rng.random_range(0..=255) as u8;
                let glyph = ascii::index_to_char(index);
                term.put_tile([x, y], Tile::new(glyph, color::BLUE, color::BLACK));
            }
        }
    }
}
