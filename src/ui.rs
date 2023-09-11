use std::rc::Rc;
use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color as TuiColor, Modifier, Style},
    widgets::{
        Block,
        Borders,
        BorderType,
        Cell as TuiCell,
        List,
        ListItem,
        Padding,
        Paragraph,
        Row,
        Table
    },
    Frame,
};

use crate::{
    game::{Game, Stats},
    config::{ConfigBuilder, GameMode, Section},
    level::Level,
    matrix::{Color, Matrix, Cell},
    piece::Piece,
};

const BOARD_WIDTH: u16 = 22;
const BOARD_HEIGHT: u16 = 22;
const LEFT_WIDGET_WIDTH: u16 = 22;
const STATS_HEIGHT: u16 = BOARD_HEIGHT / 4;
const TTRYS: &str = r#"
_____ _____  __   __
|_   _|_   _| \ \ / /__
  | |   | || '_\ V (_-<
  |_|   |_||_|  |_|/__/

"#;
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
    let next_piece = next_piece_widget(game);
    f.render_widget(next_piece, stats_chunks[0]);

    let score = score_widget(&game.stats);
    f.render_widget(score, stats_chunks[1]);

    let level = level_widget(&game.level);
    f.render_widget(level, stats_chunks[2]);

    let lines = lines_widget(&game.stats);
    f.render_widget(lines, stats_chunks[3]);
}

fn draw_top_banner<B: Backend>(f: &mut Frame<B>, target: Rect, game: &Game) {
    f.render_widget(banner_widget(game), target);
}

fn board_widget(board: &Matrix<Cell>) -> Table {
    let rows = board
        .rows
        .iter()
        .rev()
        .map(|row| {
            let cells = row
                .iter()
                .map(|cell| {
                    if cell.value == Color::White {
                        TuiCell::from("  ")
                    } else {
                        TuiCell::from("[]")
                            .style(Style::default().bg(cell_color(cell)))
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

fn next_piece_widget(game: &Game) -> Table {
    let color = game.piece_color(&game.next_piece);
    let bbox: Matrix<Cell> = generate_bounding_matrix(&game.next_piece, color);
    let table_block = Block::default()
        .borders(Borders::ALL)
        .padding(Padding { left: 6, right: 0, top: 1, bottom: 0 })
        .title("Next Piece")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(TuiColor::White));
    let rows = bbox
        .rows
        .iter()
        .rev()
        .map(|row| {
            let cells = row
                .iter()
                .map(|cell| {
                    if cell.value == Color::White {
                        TuiCell::from("  ")
                    } else {
                        TuiCell::from("[]")
                            .style(Style::default().bg(cell_color(cell)))
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

fn cell_color(cell: &Cell) -> TuiColor {
    TuiColor::Indexed(cell.color)
}

fn generate_bounding_matrix(piece: &Piece, color: u8) -> Matrix<Cell> {
    let (lower_left, upper_right) = piece.bounds();
    let width = upper_right.x - lower_left.x + 1;
    let height = upper_right.y - lower_left.y + 1;
    let mut out: Matrix<Cell> = Matrix::new(width, height, Cell::white());
    piece
        .points
        .iter()
        .for_each(|p| {
            let x = p.x - lower_left.x;
            let y = p.y - lower_left.y;
            out.set(x, y, Cell::black(color));
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
        .fg(TuiColor::Indexed(124));
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
        .fg(TuiColor::Indexed(185));
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
        .fg(TuiColor::Indexed(130));
    Paragraph::new(format!("\n{}", stats.rows_cleared))
        .block(block)
        .alignment(Alignment::Center)
        .style(style)
}

fn banner_widget(game: &Game) -> Paragraph {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Thick);
    let mut color = TuiColor::Indexed(140);
    let mut content = TTRYS;
    if game.paused {
        color = TuiColor::Yellow;
        content = PAUSED;
    }
    if game.game_over {
        color = TuiColor::Red;
        content = GAME_OVER;
    }
    let style = Style::default()
        .add_modifier(Modifier::BOLD)
        .fg(color);
    let paragraph = Paragraph::new(content)
        .alignment(Alignment::Center)
        .style(style)
        .block(block);
    paragraph
}

pub fn draw_config<B: Backend>(f: &mut Frame<B>, config_builder: &mut ConfigBuilder) {
    let size = f.size();
    let margin = (size.width - BOARD_WIDTH * 2) / 2;
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
            Constraint::Length(BOARD_WIDTH * 2),
            Constraint::Length(margin),
        ].as_ref())
        .split(chunks[1]);
    let widget_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ].as_ref())
        .split(center_chunks[1]);

    render_game_type_widget(config_builder, widget_chunks[0], f);
    render_choose_level_widget(config_builder, widget_chunks[1], f);
    render_start(config_builder, widget_chunks[2], f);
}

fn render_game_type_widget<B: Backend>(config_builder: &mut ConfigBuilder, target: Rect, f: &mut Frame<B>) {
    let border_type = if config_builder.current_section == Section::ChooseGameMode {
        BorderType::Thick
    } else {
        BorderType::Plain
    };
    let style = Style::default()
        .fg(TuiColor::Indexed(88));
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(border_type)
        .style(style)
        .title("Game Type")
        .title_alignment(Alignment::Center);
    f.render_widget(block, target);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ].as_ref())
        .split(target);

    let (
        (a_type_block_border_type, a_type_text_style),
        (b_type_block_border_type, b_type_text_style)
    ) = match config_builder.game_mode {
        GameMode::AType => (
            (BorderType::Thick, Style::default().add_modifier(Modifier::BOLD)),
            (BorderType::Plain, Style::default())
        ),
        GameMode::BType => (
            (BorderType::Plain, Style::default()),
            (BorderType::Thick, Style::default().add_modifier(Modifier::BOLD))
        )
    };
    let a_type_block = Block::default()
        .padding(Padding { left: 0, right: 0, top: 2, bottom: 2 })
        .borders(Borders::ALL)
        .border_type(a_type_block_border_type);
    let a_type = Paragraph::new("A Type")
        .block(a_type_block)
        .style(a_type_text_style)
        .alignment(Alignment::Center);
    let b_type_block = Block::default()
        .padding(Padding { left: 0, right: 0, top: 2, bottom: 2 })
        .borders(Borders::ALL)
        .border_type(b_type_block_border_type);
    let b_type = Paragraph::new("B Type")
        .block(b_type_block)
        .style(b_type_text_style)
        .alignment(Alignment::Center);
    f.render_widget(a_type, chunks[0]);
    f.render_widget(b_type, chunks[1]);

}

fn render_choose_level_widget<B: Backend>(config_builder: &mut ConfigBuilder, target: Rect, f: &mut Frame<B>) {
    let base_style = Style::default().fg(TuiColor::Indexed(220));
    let (border_type, style) = if config_builder.current_section == Section::ChooseInitialLevel {
        (BorderType::Thick, base_style.add_modifier(Modifier::BOLD))
    } else {
        (BorderType::Plain, base_style)
    };
    let container = Block::default()
        .borders(Borders::ALL)
        .border_type(border_type)
        .style(style)
        .title("Starting Level");
    let items: Vec<ListItem> = config_builder
        .level_list
        .items
        .iter()
        .map(|i| ListItem::new(*i).style(base_style))
        .collect();
    let items = List::new(items)
        .block(container)
        .highlight_symbol("> ");

    f.render_stateful_widget(items, target, &mut config_builder.level_list.state);
}

fn render_start<B: Backend>(config_builder: &mut ConfigBuilder, target: Rect, f: &mut Frame<B>) {
    let (border_type, text_style) = if config_builder.current_section == Section::StartGame {
        (BorderType::Thick, Style::default().add_modifier(Modifier::BOLD))
    } else {
        (BorderType::Plain, Style::default())
    };
    let style = Style::default()
        .fg(TuiColor::Indexed(35));
    let block = Block::default()
        .padding(Padding { left: 0, right: 0, top: 2, bottom: 2 })
        .borders(Borders::ALL)
        .border_type(border_type)
        .style(style);
    let start = Paragraph::new("Start")
        .alignment(Alignment::Center)
        .block(block)
        .style(text_style);
    f.render_widget(start, target);
}
