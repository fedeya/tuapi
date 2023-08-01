use std::io::Stdout;

use ratatui::{
    prelude::{Alignment, Constraint, CrosstermBackend, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, AppBlock, InputMode, RequestMethod};

fn selectable_block(block: AppBlock, app: &App) -> Block {
    let is_selected = block == app.selected_block;

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

    let endpoint_p = Paragraph::new(app.endpoint.as_str())
        .block(selectable_block(AppBlock::Endpoint, app).title("Endpoint"));

    let method_p = Paragraph::new(app.method.to_string())
        .block(selectable_block(AppBlock::Method, app))
        .style(Style::default().fg(match app.method {
            RequestMethod::Get => Color::Green,
            RequestMethod::Post => Color::Blue,
            RequestMethod::Put => Color::Yellow,
            RequestMethod::Delete => Color::Red,
        }))
        .alignment(Alignment::Center);

    let raw_body_p = Paragraph::new(app.raw_body.as_str())
        .block(selectable_block(AppBlock::Request, app).title("Body"));

    let help_p = Paragraph::new("Press 'q' to quit")
        .block(Block::default().borders(Borders::ALL).title("Help"));

    frame.render_widget(method_p, header_chunks[0]);
    frame.render_widget(endpoint_p, header_chunks[1]);

    frame.render_widget(raw_body_p, content_chunks[0]);

    match app.response.as_ref() {
        Some(r) => {
            let max_x =
                u16::try_from(r.text.lines().count()).unwrap() - (content_chunks[1].height - 2);

            app.response_scroll.0 = app.response_scroll.0.clamp(0, max_x);

            let response_p = Paragraph::new(r.text.clone())
                .block(selectable_block(AppBlock::Response, app).title("Response"))
                .wrap(Wrap { trim: false, })
                .scroll(app.response_scroll);

            frame.render_widget(response_p, content_chunks[1]);
        }
        None => {
            let helper_text = Paragraph::new("Press <Enter> to send request")
                .alignment(Alignment::Center)
                .block(selectable_block(AppBlock::Response, app).title("Response"));

            frame.render_widget(helper_text, content_chunks[1]);
        }
    }

    frame.render_widget(help_p, main_chunks[2]);
}
