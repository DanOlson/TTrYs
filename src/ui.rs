use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
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

pub fn draw<B: Backend>(f: &mut Frame<B>, game: &mut Game) {
    let size = f.size();
    let margin = (size.width - LEFT_WIDGET_WIDTH - BOARD_WIDTH) / 2;
    let vertical_margin = (size.height - BOARD_HEIGHT) / 2;

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Tetris")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(margin),
            Constraint::Length(LEFT_WIDGET_WIDTH),
            Constraint::Length(BOARD_WIDTH),
            Constraint::Length(margin),
        ].as_ref())
        .split(size);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(vertical_margin),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(vertical_margin),
        ].as_ref())
        .split(chunks[1]);
    let middle_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(vertical_margin),
            Constraint::Length(BOARD_HEIGHT),
            Constraint::Length(vertical_margin),
        ].as_ref())
        .split(chunks[2]);

    let board = board_widget(&game.board);
    f.render_widget(board, middle_chunks[1]);

    let next_piece = next_piece_widget(&game.next_piece);
    f.render_widget(next_piece, left_chunks[1]);

    let score = score_widget(&game.stats);
    f.render_widget(score, left_chunks[2]);

    let level = level_widget(&game.level);
    f.render_widget(level, left_chunks[3]);
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
        .border_type(BorderType::Thick);
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
