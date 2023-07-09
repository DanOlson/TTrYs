#![allow(dead_code)]
use std::{
    thread,
    time::Duration,
    sync::mpsc,
    io::{Write, Stdout, stdout, stdin}
};
use termion::{
    raw::{IntoRawMode, RawTerminal},
    event::Key,
    input::TermRead
};
use crate::game::{Game, GameMode};

mod matrix;
mod piece;
mod game;
mod scoring;
mod rotate;

fn main() {
    let mut game = Game::new(GameMode::AType);
    run_game(&mut game).ok();
}

fn run_game(game: &mut Game) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let events = setup_events(Duration::from_millis(10));
    loop {
        match events.recv()? {
            Event::Input(key) => match key {
                Key::Ctrl('c') => exit(&mut stdout),
                Key::Char('s') => game.on_down(),
                Key::Char('a') => game.on_left(),
                Key::Char('d') => game.on_right(),
                Key::Left => game.on_rotate_counterclockwise(),
                Key::Right => game.on_rotate_clockwise(),
                _ => {}
            },
            Event::Tick => {
                game.on_tick();
            }
        }
        render(game, &mut stdout);
    }
}

fn render(game: &Game, out: &mut RawTerminal<Stdout>) {
    write!(
        out,
        "{}{}{}{:?}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide,
        game.board
    ).unwrap();
    out.flush().unwrap();
}

fn exit(out: &mut RawTerminal<Stdout>) {
    write!(out, "{}", termion::cursor::Show).unwrap();
    panic!("all done");
}

enum Event {
    Input(Key),
    Tick,
}

fn setup_events(tick_rate: Duration) -> std::sync::mpsc::Receiver<Event> {
    let (tx, rx) = mpsc::channel::<Event>();
    let keys_tx = tx.clone();

    thread::spawn(move || loop {
        if let Err(err) = tx.send(Event::Tick) {
            eprintln!("{err}");
            break;
        }

        thread::sleep(tick_rate);
    });

    thread::spawn(move || {
        let stdin = stdin();
        for key in stdin.keys().flatten() {
            if let Err(err) = keys_tx.send(Event::Input(key)) {
                eprintln!("{err}");
                break;
            }
        }
    });

    rx
}
