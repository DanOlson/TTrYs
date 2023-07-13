use std::rc::Rc;
use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color as UiColor, Modifier, Style},
    widgets::{Block, Borders, BorderType, Cell, Padding, Paragraph, Row, Table},
    Frame,
};

use crate::{
    game::{Game, Stats, Level},
    matrix::{Color, Matrix},
    piece::Piece,
};

const BOARD_WIDTH: u16 = 22;
const BOARD_HEIGHT: u16 = 22;
const LEFT_WIDGET_WIDTH: u16 = 22;
const STATS_HEIGHT: u16 = BOARD_HEIGHT / 4;
const GAME_OVER: &str = r#"
  ___   _   __  __ ___    _____   _____ ___
 / __| /_\ |  \/  | __|  / _ \ \ / / __| _ \
| (_ |/ _ \| |\/| | _|  | (_) \ V /| _||   /
 \___/_/ \_\_|  |_|___|  \___/ \_/ |___|_|_\

"#;
const PAUSED: &str = r#"
 ___  _  _   _ ___ ___
| _ \/_\| | | / __| __|
|  _/ _ \ |_| \__ \ _|
|_|/_/ \_\___/|___/___|

"#;

pub fn draw<B: Backend>(f: &mut Frame<B>, game: &mut Game) {
    let size = f.size();
    let margin = (size.width - LEFT_WIDGET_WIDTH - BOARD_WIDTH) / 2;
    let vertical_margin = (size.height - BOARD_HEIGHT) / 2;

    let block = Block::default()
        .borders(Borders::ALL)
        .title("TTrYs")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Double);

    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(vertical_margin),
            Constraint::Length(BOARD_HEIGHT),
            Constraint::Length(vertical_margin),
        ].as_ref())
        .split(size);

    let center_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(margin),
            Constraint::Length(LEFT_WIDGET_WIDTH),
            Constraint::Length(BOARD_WIDTH),
            Constraint::Length(margin),
        ].as_ref())
        .split(chunks[1]);

    let stats_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(STATS_HEIGHT),
            Constraint::Length(STATS_HEIGHT),
            Constraint::Length(STATS_HEIGHT),
            Constraint::Length(STATS_HEIGHT),
            Constraint::Min(0),
        ].as_ref())
        .split(center_chunks[1]);

    draw_game_board(f, center_chunks[2], game);
    draw_stats_widgets(f, stats_chunks, game);
    draw_top_banner(f, chunks[0], game);
}

fn draw_game_board<B: Backend>(f: &mut Frame<B>, target: Rect, game: &Game) {
    let board = board_widget(&game.board);
    f.render_widget(board, target);
}

fn draw_stats_widgets<B: Backend>(f: &mut Frame<B>, stats_chunks: Rc<[Rect]>, game: &Game) {
    let next_piece = next_piece_widget(&game.next_piece);
    f.render_widget(next_piece, stats_chunks[0]);

    let score = score_widget(&game.stats);
    f.render_widget(score, stats_chunks[1]);

    let level = level_widget(&game.level);
    f.render_widget(level, stats_chunks[2]);

    let lines = lines_widget(&game.stats);
    f.render_widget(lines, stats_chunks[3]);
}

fn draw_top_banner<B: Backend>(f: &mut Frame<B>, target: Rect, game: &Game) {
    if let Some(banner) = banner_widget(game) {
        f.render_widget(banner, target);
    }
}

fn board_widget(board: &Matrix<Color>) -> Table {
    let rows = board
        .rows
        .iter()
        .rev()
        .map(|row| {
            let cells = row
                .iter()
                .map(|color| {
                    if color == &Color::White {
                        Cell::from("  ")
                    } else {
                        Cell::from("[]")
                            .style(Style::default().bg(UiColor::Gray))
                    }
                });
            Row::new(cells).height(1)
        });
    let table_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Thick);

    Table::new(rows)
        .block(table_block)
        .widths(&[
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
        ])
        .column_spacing(0)
}

fn next_piece_widget(next_piece: &Piece) -> Table {
    let bbox = generate_bounding_matrix(next_piece);
    let table_block = Block::default()
        .borders(Borders::ALL)
        .padding(Padding { left: 6, right: 0, top: 1, bottom: 0 })
        .title("Next Piece")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(UiColor::LightGreen));
    let rows = bbox
        .rows
        .iter()
        .rev()
        .map(|row| {
            let cells = row
                .iter()
                .map(|color| {
                    if color == &Color::White {
                        Cell::from("  ")
                    } else {
                        Cell::from("[]")
                            .style(Style::default().bg(UiColor::Gray))
                    }
                });
            Row::new(cells).height(1)
        });
    Table::new(rows)
        .block(table_block)
        .widths(&[
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
        ])
        .column_spacing(0)
}

fn generate_bounding_matrix(piece: &Piece) -> Matrix<Color> {
    let (lower_left, upper_right) = piece.bounds();
    let width = upper_right.x - lower_left.x + 1;
    let height = upper_right.y - lower_left.y + 1;
    let mut out = Matrix::new(width, height, Color::White);
    piece
        .points
        .iter()
        .for_each(|p| {
            let x = p.x - lower_left.x;
            let y = p.y - lower_left.y;
            out.set(x, y, Color::Black);
        });
    out
}

fn score_widget(stats: &Stats) -> Paragraph {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .title("Score")
        .title_alignment(Alignment::Center);
    let style = Style::default()
        .add_modifier(Modifier::BOLD)
        .fg(UiColor::LightYellow);
    Paragraph::new(format!("\n{}", stats.score))
        .block(block)
        .alignment(Alignment::Center)
        .style(style)
}

fn level_widget(level: &Level) -> Paragraph {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .title("Level")
        .title_alignment(Alignment::Center);
    let style = Style::default()
        .add_modifier(Modifier::BOLD)
        .fg(UiColor::LightCyan);
    Paragraph::new(format!("\n{}", level.number))
        .block(block)
        .alignment(Alignment::Center)
        .style(style)
}

fn lines_widget(stats: &Stats) -> Paragraph {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .title("Lines")
        .title_alignment(Alignment::Center);
    let style = Style::default()
        .add_modifier(Modifier::BOLD)
        .fg(UiColor::LightRed);
    Paragraph::new(format!("\n{}", stats.rows_cleared))
        .block(block)
        .alignment(Alignment::Center)
        .style(style)
}

fn banner_widget(game: &Game) -> Option<Paragraph> {
    if !game.paused && !game.game_over { return None }

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Thick);
    let style = Style::default()
        .add_modifier(Modifier::BOLD)
        .fg(if game.paused { UiColor::Yellow } else { UiColor::Red });
    let content = if game.paused { PAUSED } else { GAME_OVER };
    let paragraph = Paragraph::new(content)
        .alignment(Alignment::Center)
        .style(style)
        .block(block);
    Some(paragraph)
}
