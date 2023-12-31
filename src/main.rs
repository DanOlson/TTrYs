#![allow(dead_code)]
use std::{
    thread,
    time::Duration,
    sync::mpsc,
    io::{stdout, stdin}
};
use termion::{
    raw::IntoRawMode,
    event::Key,
    input::TermRead,
    screen::IntoAlternateScreen,
};
use ratatui::{
    backend::{Backend, TermionBackend},
    Terminal,
};

use crate::{
    config::Config,
    game::Game,
};

mod config;
mod game;
mod ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = stdout()
        .into_raw_mode()?
        .into_alternate_screen()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let events = setup_events(Duration::from_millis(10));
    let config = configure(&mut terminal, &events)?;
    let mut game = Game::new(config);
    run_game(&mut game, &mut terminal, &events)?;

    Ok(())
}

fn configure<B: Backend>(
    terminal: &mut Terminal<B>,
    events: &mpsc::Receiver<Event>
) -> Result<Config, Box<dyn std::error::Error>> {
    let mut config_builder = Config::builder();

    loop {
        terminal.draw(|f| ui::draw_config(f, &mut config_builder))?;

        if config_builder.is_configured() { return Ok(config_builder.build()) }
        if let Event::Input(key) = events.recv()? {
            match key {
                Key::Char('w') => config_builder.previous_section(),
                Key::Char('s') => config_builder.next_section(),
                Key::Char('a') => config_builder.on_left(),
                Key::Char('d') => config_builder.on_right(),
                Key::Char('\n') => config_builder.configured(),
                _ => {}
            }
        };
    }
}

fn run_game<B: Backend>(
    game: &mut Game,
    terminal: &mut Terminal<B>,
    events: &mpsc::Receiver<Event>
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|f| ui::draw(f, game))?;

        if game.should_quit() { return Ok(()) }
        match events.recv()? {
            Event::Input(key) => match key {
                Key::Ctrl('c') => game.quit(),
                Key::Char('s') => game.on_down(),
                Key::Char('a') => game.on_left(),
                Key::Char('d') => game.on_right(),
                Key::Char(' ') => game.toggle_pause(),
                Key::Left => game.on_rotate_counterclockwise(),
                Key::Right => game.on_rotate_clockwise(),
                _ => {}
            },
            Event::Tick => {
                game.on_tick();
            }
        }
    }
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
