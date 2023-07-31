use std::io::Stdout;

use ratatui::{
    prelude::{Alignment, Constraint, CrosstermBackend, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{App, RequestMethod};

fn selectable_block(app: &App) -> Block {
    Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
}

pub fn draw(frame: &mut Frame<CrosstermBackend<Stdout>>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(0), Constraint::Min(0)])
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

    let endpoint_p =
        Paragraph::new(app.endpoint.as_str()).block(selectable_block(app).title("Endpoint"));

    let method_p = Paragraph::new(app.method.to_string())
        .block(selectable_block(app))
        .style(Style::default().fg(match app.method {
            RequestMethod::Get => Color::Green,
            RequestMethod::Post => Color::Blue,
            RequestMethod::Put => Color::Yellow,
            RequestMethod::Delete => Color::Red,
        }))
        .alignment(Alignment::Center);

    let body_block = selectable_block(app).title("Body");

    let response_p = Paragraph::new("").block(selectable_block(app).title("Response"));

    let help_p = Paragraph::new("Press 'q' to quit").block(selectable_block(app).title("Help"));

    frame.render_widget(method_p, header_chunks[0]);
    frame.render_widget(endpoint_p, header_chunks[1]);

    frame.render_widget(body_block, content_chunks[0]);
    frame.render_widget(response_p, content_chunks[1]);

    frame.render_widget(help_p, main_chunks[2]);
}
