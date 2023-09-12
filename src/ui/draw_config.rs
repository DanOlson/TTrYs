use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color as TuiColor, Modifier, Style},
    widgets::{
        Block,
        Borders,
        BorderType,
        List,
        ListItem,
        Padding,
        Paragraph,
    },
    Frame,
};

use crate::config::{ConfigBuilder, GameMode, Section};

const BOARD_WIDTH: u16 = 22;
const BOARD_HEIGHT: u16 = 22;
const LEFT_WIDGET_WIDTH: u16 = 22;
const STATS_HEIGHT: u16 = BOARD_HEIGHT / 4;

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
