mod input;
mod popup;
mod request_tab;
mod syntax;

use std::io::Stdout;

use popup::render_popup;
use ratatui::{
    prelude::{Alignment, Constraint, CrosstermBackend, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};
use request_tab::render_request_tab;

use crate::app::{App, AppBlock, InputMode, OrderNavigation, RequestMethod};

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

    let request_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(content_chunks[0]);

    let response_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(content_chunks[1]);

    let request_tabs = vec![
        Span::styled("Body", Style::default().fg(Color::White)),
        Span::styled("Query", Style::default().fg(Color::White)),
        Span::styled("Headers", Style::default().fg(Color::White)),
        // Span::styled("Auth", Style::default().fg(Color::White)),
        // Span::styled("Cookies", Style::default().fg(Color::White)),
    ];

    let tab = Tabs::new(request_tabs)
        .block(selectable_block(AppBlock::Request, app))
        .divider(Span::raw("|"))
        .select(app.request_tab.clone().get_index())
        .highlight_style(Style::default().fg(Color::Green));

    frame.render_widget(method_p, header_chunks[0]);
    frame.render_widget(endpoint_input, header_chunks[1]);

    frame.render_widget(tab, request_chunks[0]);

    render_request_tab(app, frame, request_chunks.to_vec());

    render_response(app, frame, response_chunks.to_vec());

    frame.render_widget(help_p, main_chunks[2]);

    if let Some(_) = app.popup {
        render_popup(app, frame);
    }
}

fn render_response(
    app: &mut App,
    frame: &mut Frame<CrosstermBackend<Stdout>>,
    response_chunks: Vec<ratatui::prelude::Rect>,
) {
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
