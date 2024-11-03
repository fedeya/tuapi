use std::io::Stdout;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::CrosstermBackend,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{App, AppBlock};

use super::{selectable_block, syntax};

pub fn render_response(app: &mut App, frame: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
    let response_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(area);

    match app.response.as_ref() {
        Some(r) => {
            let lines_count = u16::try_from(r.text.lines().count()).unwrap_or(1);
            let max_x = if lines_count > response_chunks[0].height {
                lines_count - (response_chunks[0].height - 2)
            } else {
                0
            };

            app.response_scroll.0 = app.response_scroll.0.clamp(0, max_x);

            let lines = syntax::highlight_response(r.text.clone(), r.content_type.clone());

            let response_p = Paragraph::new(lines)
                .block(selectable_block(AppBlock::Response, app).title("Response"))
                // .wrap(Wrap { trim: false })
                .scroll(app.response_scroll);

            let status_code_style = Style::default().fg(match r.status_code {
                200..=299 => Color::Green,
                300..=399 => Color::Blue,
                400..=499 => Color::Yellow,
                500..=599 => Color::Red,
                _ => Color::White,
            });

            let status_code_text = if app.is_loading {
                "Loading...".to_string()
            } else {
                r.status_code.to_string()
            };

            let status_code_p = Paragraph::new(status_code_text)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(status_code_style),
                )
                .style(status_code_style)
                .alignment(Alignment::Center);

            frame.render_widget(response_p, response_chunks[0]);
            frame.render_widget(status_code_p, response_chunks[1]);
        }
        None => {
            let helper_text = Paragraph::new("Created with love by @fedeya")
                .alignment(Alignment::Center)
                .block(selectable_block(AppBlock::Response, app).title("Response"));

            let status_blank = Paragraph::new(if app.is_loading {
                "Loading..."
            } else {
                "Press <Enter> to send request"
            })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::White)),
            )
            .alignment(Alignment::Center);

            frame.render_widget(helper_text, response_chunks[0]);
            frame.render_widget(status_blank, response_chunks[1]);
        }
    }
}
