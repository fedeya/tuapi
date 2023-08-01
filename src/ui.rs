use cached::proc_macro::cached;
use once_cell::sync::Lazy;
use std::io::Stdout;

use ratatui::{
    prelude::{Alignment, Constraint, CrosstermBackend, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use syntect::{
    easy::HighlightLines, highlighting::ThemeSet, parsing::SyntaxSet, util::LinesWithEndings,
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

    let between_cursor = app.endpoint.split_at(app.input_cursor_position.into());

    let endpoint_line = Line::from(vec![
        Span::raw(between_cursor.0),
        match app.input_mode {
            InputMode::Insert => Span::styled(
                match between_cursor.1.get(0..1) {
                    Some(c) => c,
                    None => " ",
                },
                Style::default().bg(Color::Green).fg(Color::Black),
            ),
            InputMode::Normal => Span::raw(match between_cursor.1.get(0..1) {
                Some(c) => c,
                None => " ",
            }),
        },
        match between_cursor.1.get(1..) {
            Some(c) => Span::raw(c),
            None => Span::raw(""),
        },
    ]);

    let endpoint_p = Paragraph::new(endpoint_line)
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

    let help_p = Paragraph::new("Press 'q' to quit").block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title("Help"),
    );

    frame.render_widget(method_p, header_chunks[0]);
    frame.render_widget(endpoint_p, header_chunks[1]);

    frame.render_widget(raw_body_p, content_chunks[0]);

    match app.response.as_ref() {
        Some(r) => {
            let lines_count = u16::try_from(r.text.lines().count()).unwrap_or(1);
            let max_x = if lines_count > content_chunks[1].height {
                lines_count - (content_chunks[1].height - 2)
            } else {
                0
            };

            app.response_scroll.0 = app.response_scroll.0.clamp(0, max_x);

            let lines = highlight_response(r.text.clone());

            let response_p = Paragraph::new(lines)
                .block(selectable_block(AppBlock::Response, app).title("Response"))
                .wrap(Wrap { trim: false })
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

static PS: Lazy<SyntaxSet> = Lazy::new(SyntaxSet::load_defaults_newlines);
static TS: Lazy<ThemeSet> = Lazy::new(ThemeSet::load_defaults);

#[cached]
fn highlight_response(response: String) -> Vec<Line<'static>> {
    let syntax = PS.find_syntax_by_extension("json").unwrap();
    let mut h = HighlightLines::new(syntax, &TS.themes["base16-ocean.dark"]);

    let mut lines: Vec<Line> = Vec::new();

    for line in LinesWithEndings::from(response.as_str()) {
        let ranges: Vec<(syntect::highlighting::Style, &str)> =
            h.highlight_line(line, &PS).unwrap();

        let spans: Vec<Span> = ranges
            .iter()
            .map(|segment| {
                let (style, content) = segment;

                Span::styled(
                    content.to_string(),
                    Style {
                        fg: translate_colour(style.foreground),
                        ..Style::default()
                    },
                )
            })
            .collect();

        lines.push(Line::from(spans));
    }

    lines
}

fn translate_colour(syntect_color: syntect::highlighting::Color) -> Option<Color> {
    match syntect_color {
        syntect::highlighting::Color { r, g, b, a } if a > 0 => Some(Color::Rgb(r, g, b)),
        _ => None,
    }
}
