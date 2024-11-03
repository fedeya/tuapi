mod input;
mod popup;
mod request_tab;
mod response;
mod syntax;

use std::io::Stdout;

use popup::render_popup;
use ratatui::{
    prelude::{Alignment, Constraint, CrosstermBackend, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use request_tab::render_request_tab;
use response::render_response;

use crate::app::{App, AppBlock, InputMode, RequestMethod};

use self::input::create_input;

fn selectable_block(block: AppBlock, app: &App) -> Block {
    let is_selected = block == app.selected_block && app.popup.is_none();

    let border_style = Style::default().fg(if is_selected && app.input_mode == InputMode::Insert {
        Color::Green
    } else if is_selected {
        Color::Blue
    } else {
        Color::White
    });

    Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .style(Style::default().fg(Color::White))
}

pub fn draw(frame: &mut Frame<CrosstermBackend<Stdout>>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(0), Constraint::Min(0)])
        .horizontal_margin(1)
        .split(frame.size());

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(chunks[1]);

    let method_size = u16::try_from(app.method.to_string().len()).unwrap() + 4;

    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(method_size), Constraint::Min(0)])
        .split(main_chunks[0]);

    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[1]);

    let endpoint_input = create_input(&app.endpoint, app, app.selected_block == AppBlock::Endpoint)
        .block(selectable_block(AppBlock::Endpoint, app).title("Endpoint"));

    let method_p = Paragraph::new(app.method.to_string())
        .block(selectable_block(AppBlock::Method, app))
        .style(Style::default().fg(match app.method {
            RequestMethod::Get => Color::Green,
            RequestMethod::Post => Color::Blue,
            RequestMethod::Put => Color::Yellow,
            RequestMethod::Delete => Color::Red,
            RequestMethod::Patch => Color::Magenta,
        }))
        .alignment(Alignment::Center);

    let help_p = Paragraph::new("Press 'q' to quit").block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title("Help"),
    );

    frame.render_widget(method_p, header_chunks[0]);
    frame.render_widget(endpoint_input, header_chunks[1]);

    render_request_tab(app, frame, content_chunks[0]);

    render_response(app, frame, content_chunks[1]);

    frame.render_widget(help_p, main_chunks[2]);

    if let Some(_) = app.popup {
        render_popup(app, frame);
    }
}
