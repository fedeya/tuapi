use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::app::{App, Input, InputMode};

pub fn create_input<'a>(input: &'a Input, app: &App) -> Paragraph<'a> {
    let (left, right) = input.text.split_at(input.cursor_position.into());

    let line = Line::from(vec![
        Span::raw(left),
        Span::styled(
            match right.get(0..1) {
                Some(c) => c,
                None => " ",
            },
            match app.input_mode {
                InputMode::Insert => Style::default().bg(Color::Green).fg(Color::Black),
                InputMode::Normal => Style::default(),
            },
        ),
        match right.get(1..) {
            Some(c) => Span::raw(c),
            None => Span::raw(""),
        },
    ]);

    Paragraph::new(line)
}
