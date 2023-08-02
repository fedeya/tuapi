use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::app::{App, AppBlock, Input, InputMode};

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
    let lines = input
        .text
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let is_cursor_in_line = index == usize::from(input.cursor_position.y);

            if !is_cursor_in_line {
                return Line::from(vec![Span::raw(line)]);
            }

            let (left, right) = line.split_at(input.cursor_position.x.into());

            Line::from(vec![
                Span::raw(left),
                Span::styled(
                    match right.get(0..1) {
                        Some(c) => c,
                        None => " ",
                    },
                    match app.input_mode {
                        InputMode::Insert if app.selected_block == AppBlock::RequestContent => {
                            Style::default().bg(Color::Green).fg(Color::Black)
                        }
                        _ => Style::default(),
                    },
                ),
                match right.get(1..) {
                    Some(c) => Span::raw(c),
                    None => Span::raw(""),
                },
            ])
        })
        .collect::<Vec<Line>>();

    Paragraph::new(lines)
}
