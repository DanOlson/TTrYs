use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color as UiColor, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Block, Borders, BorderType, Cell, Padding, Paragraph, Row, Table},
    Frame,
};

use crate::{
    game::{Game, Stats, Level},
    matrix::{Color, Matrix},
    piece::Piece,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, game: &mut Game) {
    let size = f.size();

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Tetris")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(40),
            Constraint::Length(31),
            Constraint::Min(40),
        ].as_ref())
        .split(size);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(80),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
        ].as_ref())
        .split(chunks[0]);
    let middle_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(70),
            Constraint::Percentage(10),
        ].as_ref())
        .split(chunks[1]);
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ].as_ref())
        .split(chunks[2]);

    // let game_board_block = Block::default()
    //     .borders(Borders::ALL);

    // let rendered_game = format!("{}", game.board);
    // let board = Paragraph::new(rendered_game).block(game_board_block);
    let board = board_widget(&game.board);
    f.render_widget(board, middle_chunks[1]);

    let next_piece = next_piece_widget(&game.next_piece);
    f.render_widget(next_piece, right_chunks[0]);

    let score = score_widget(&game.stats);
    f.render_widget(score, left_chunks[1]);

    let level = level_widget(&game.level);
    f.render_widget(level, left_chunks[2]);
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
        .borders(Borders::ALL);

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
}

fn next_piece_widget(next_piece: &Piece) -> Table {
    let bbox = generate_bounding_matrix(next_piece);
    let table_block = Block::default()
        .borders(Borders::ALL)
        .title("Next Piece")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
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
        .title("Score")
        .title_alignment(Alignment::Center);
    let style = Style::default()
        .add_modifier(Modifier::BOLD)
        .fg(UiColor::LightYellow);
    Paragraph::new(format!("{}", stats.score))
        .block(block)
        .alignment(Alignment::Center)
        .style(style)
}

fn level_widget(level: &Level) -> Paragraph {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Level")
        .title_alignment(Alignment::Center);
    let style = Style::default()
        .add_modifier(Modifier::BOLD)
        .fg(UiColor::LightCyan);
    Paragraph::new(format!("{}", level.number))
        .block(block)
        .alignment(Alignment::Center)
        .style(style)
}
