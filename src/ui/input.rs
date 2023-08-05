use std::io::stderr;

use crossterm::cursor::position;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};
use syntect::{easy::HighlightLines, util::LinesWithEndings};

use crate::app::{App, AppBlock, Input, InputMode};

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

    for (index, line) in LinesWithEndings::from(input.text.as_str()).enumerate() {
        let ranges = h.highlight_line(line, &PS).unwrap();

        let is_cursor_in_line = index == usize::from(input.cursor_position.y);

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

                eprintln!("cursor position: {:?}", input.cursor_position.x);
                eprintln!("range: {:?} line: {:?}", i, index);
                eprintln!("content: ({:?})", content);

                let is_cursor_in_segment = input.cursor_position.x as usize >= position.0
                    && position.1 > input.cursor_position.x as usize;

                eprintln!("is_cursor_in_segment: {:?}", is_cursor_in_segment);
                eprintln!("\n");

                if !is_cursor_in_segment {
                    vec![Span::styled(
                        content.to_string(),
                        Style {
                            fg: translate_colour(style.foreground),
                            ..Style::default()
                        },
                    )]
                } else {
                    let current = input.cursor_position.x as usize - position.0;
                    let (left, right) = content.split_at(current);

                    eprintln!("left: {:?} right: {:?}", left, right);
                    eprintln!("current: {:?}", current);

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
                                Some(c) => {
                                    if c == "\n" {
                                        " "
                                    } else {
                                        c
                                    }
                                }
                                None => " ",
                            },
                            match app.input_mode {
                                InputMode::Insert
                                    if app.selected_block == AppBlock::RequestContent =>
                                {
                                    Style::default().bg(Color::Green).fg(Color::Black)
                                }
                                _ => Style::default(),
                            },
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

    // let lines = input
    //     .text
    //     .lines()
    //     .enumerate()
    //     .map(|(index, line)| {
    //         let is_cursor_in_line = index == usize::from(input.cursor_position.y);
    //
    //         if !is_cursor_in_line {
    //             return Line::from(vec![Span::raw(line)]);
    //         }
    //
    //         let (left, right) = line.split_at(input.cursor_position.x.into());
    //
    //         Line::from(vec![
    //             Span::raw(left),
    //             Span::styled(
    //                 match right.get(0..1) {
    //                     Some(c) => c,
    //                     None => " ",
    //                 },
    //                 match app.input_mode {
    //                     InputMode::Insert if app.selected_block == AppBlock::RequestContent => {
    //                         Style::default().bg(Color::Green).fg(Color::Black)
    //                     }
    //                     _ => Style::default(),
    //                 },
    //             ),
    //             match right.get(1..) {
    //                 Some(c) => Span::raw(c),
    //                 None => Span::raw(""),
    //             },
    //         ])
    //     })
    //     .collect::<Vec<Line>>();

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

    eprintln!(
        "start: {} end: {} index: {index}",
        current,
        current + vector[index].1.len()
    );

    return (current, current + vector[index].1.len());
}
