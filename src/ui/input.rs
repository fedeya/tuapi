use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};
use syntect::easy::HighlightLines;

use crate::app::{App, AppBlock, InputMode};
use crate::event::input::Input;

use super::syntax::{translate_colour, PS, TS};

pub fn create_input<'a>(input: &'a Input, app: &App) -> Paragraph<'a> {
    let (left, right) = input.text.split_at(input.cursor_position.x.into());

    let line = Line::from(vec![
        Span::raw(left),
        Span::styled(
            match right.get(0..1) {
                Some(c) => c,
                None => " ",
            },
            match app.input_mode {
                InputMode::Insert if app.selected_block == AppBlock::Endpoint => {
                    Style::default().bg(Color::Green).fg(Color::Black)
                }
                _ => Style::default(),
            },
        ),
        match right.get(1..) {
            Some(c) => Span::raw(c),
            None => Span::raw(""),
        },
    ]);

    Paragraph::new(line)
}

pub fn create_textarea<'a>(input: &'a Input, app: &App) -> Paragraph<'a> {
    let syntax = PS.find_syntax_by_extension("json").unwrap();
    let mut h = HighlightLines::new(syntax, &TS.themes["base16-ocean.dark"]);

    let mut lines = Vec::new();

    for (index, line) in input.text.split("\n").enumerate() {
        let ranges = h.highlight_line(line, &PS).unwrap();

        let is_cursor_in_line = index == usize::from(input.cursor_position.y);

        if ranges.is_empty() && is_cursor_in_line {
            lines.push(Line::from(Span::styled(
                " ",
                match app.input_mode {
                    InputMode::Insert if app.selected_block == AppBlock::RequestContent => {
                        Style::default().bg(Color::Green).fg(Color::Black)
                    }
                    _ => Style::default(),
                },
            )));

            continue;
        }

        if !is_cursor_in_line {
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

            continue;
        }

        let spans: Vec<Vec<Span>> = ranges
            .iter()
            .enumerate()
            .map(|(i, segment)| {
                let (style, content) = segment;

                let position = get_current_position(&ranges, i);

                let is_cursor_in_segment = input.cursor_position.x as usize >= position.0
                    && position.1 > input.cursor_position.x as usize;

                let cursor_styles = match app.input_mode {
                    InputMode::Insert if app.selected_block == AppBlock::RequestContent => {
                        Style::default().bg(Color::Green).fg(Color::Black)
                    }
                    _ => Style::default(),
                };

                if !is_cursor_in_segment {
                    let is_last_segment =
                        i == ranges.len() - 1 && input.cursor_position.x as usize == position.1;
                    let is_first_segment = i == 0 && input.cursor_position.x as usize == position.0;

                    vec![
                        match is_first_segment {
                            true => Span::styled(" ", cursor_styles),
                            false => Span::raw(""),
                        },
                        Span::styled(
                            content.to_string(),
                            Style {
                                fg: translate_colour(style.foreground),
                                ..Style::default()
                            },
                        ),
                        match is_last_segment {
                            true => Span::styled(" ", cursor_styles),
                            false => Span::raw(""),
                        },
                    ]
                } else {
                    let current = input.cursor_position.x as usize - position.0;
                    let (left, right) = content.split_at(current);

                    vec![
                        Span::styled(
                            left.to_string(),
                            Style {
                                fg: translate_colour(style.foreground),
                                ..Style::default()
                            },
                        ),
                        Span::styled(
                            match right.get(0..1) {
                                Some(c) => c,
                                None => " ",
                            },
                            cursor_styles,
                        ),
                        match right.get(1..) {
                            Some(c) => Span::styled(
                                c,
                                Style {
                                    fg: translate_colour(style.foreground),
                                    ..Style::default()
                                },
                            ),
                            None => Span::raw(""),
                        },
                    ]
                }
            })
            .collect();

        lines.push(Line::from(spans.concat()));
    }

    Paragraph::new(lines)
}

fn get_current_position(
    vector: &Vec<(syntect::highlighting::Style, &str)>,
    index: usize,
) -> (usize, usize) {
    let mut current = 0;

    for (_, content) in vector.get(0..index).unwrap() {
        current += content.len();
    }

    return (current, current + vector[index].1.len());
}
