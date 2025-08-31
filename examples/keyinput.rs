use bevy::{
    input::keyboard::{Key, KeyboardInput},
    prelude::*,
};
use bevy_ascii_terminal::{Terminal, TerminalCamera, TerminalPlugins, Tile, color};

const WIDTH: usize = 40;
const HEIGHT: usize = 20;

#[derive(Resource)]
struct Board {
    w: usize,
    h: usize,
    pos: (usize, usize),
    cells: Vec<char>,
}

impl Board {
    fn new(w: usize, h: usize) -> Self {
        Self {
            w,
            h,
            pos: (0, 0),
            cells: vec![' '; w * h],
        }
    }

    #[inline]
    fn idx(&self, x: usize, y: usize) -> usize {
        y * self.w + x
    }

    fn put(&mut self, c: char) {
        let idx = self.idx(self.pos.0, self.pos.1);
        self.cells[idx] = c;
        self.pos.0 += 1;
        if self.pos.0 >= self.w {
            self.enter();
        }
    }

    fn enter(&mut self) {
        self.pos.0 = 0;
        if self.pos.1 + 1 < self.h {
            self.pos.1 += 1;
        }
    }

    fn clear(&mut self) {
        for c in self.cells.iter_mut() {
            *c = ' ';
        }
        self.pos = (0, 0);
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .insert_resource(Board::new(WIDTH, HEIGHT))
        .add_systems(Startup, setup)
        .add_systems(Update, (input, draw))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Terminal::new([WIDTH, HEIGHT]),));
    commands.spawn(TerminalCamera::new());
}

fn input(mut keys: EventReader<KeyboardInput>, mut board: ResMut<Board>) {
    for key in keys.read() {
        if !key.state.is_pressed() {
            continue;
        }

        match &key.logical_key {
            Key::Character(character) => {
                if let Some(c) = character.chars().next() {
                    board.put(c.to_ascii_lowercase());
                }
            }
            Key::Backspace => board.clear(),
            Key::Enter => board.enter(),
            Key::Space => board.put(' '),
            _ => {}
        }
    }
}

fn draw(mut q_term: Query<&mut Terminal>, board: Res<Board>) {
    let mut term = q_term.single_mut().unwrap();
    term.clear();

    for y in 0..board.h {
        for x in 0..board.w {
            if board.cells[board.idx(x, y)] != ' ' {
                term.put_tile(
                    [x, (board.h - y - 1) as usize],
                    Tile::new(board.cells[board.idx(x, y)], color::WHITE, color::BLACK),
                );
            }
        }
    }
}
