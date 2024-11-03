use std::io::Stdout;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::CrosstermBackend,
    style::{Color, Style},
    widgets::{Paragraph, Row, Table, TableState},
    Frame,
};

use crate::app::{App, AppBlock, BodyContentType, BodyType, RequestTab};

use super::{input::create_textarea, selectable_block};

pub fn render_request_tab(
    app: &App,
    frame: &mut Frame<'_, CrosstermBackend<Stdout>>,
    request_chunks: Vec<Rect>,
) {
    match app.request_tab {
        RequestTab::Body => {
            let body_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)])
                .split(request_chunks[1]);

            let content_type_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(body_chunks[1]);

            let content_type_mode_p = Paragraph::new(match app.body_content_type {
                BodyContentType::Text(_) => "Text",
                BodyContentType::Form => "Form",
            })
            .block(selectable_block(AppBlock::RequestContent, app).title("Content"))
            .alignment(Alignment::Center);

            frame.render_widget(
                content_type_mode_p,
                match app.body_content_type {
                    BodyContentType::Text(_) => content_type_chunks[0],
                    BodyContentType::Form => body_chunks[1],
                },
            );

            if let BodyContentType::Text(body_type) = app.body_content_type.clone() {
                let content_type_format_p = Paragraph::new(match body_type {
                    BodyType::Json => "JSON",
                    BodyType::Raw => "Raw",
                    BodyType::Xml => "XML",
                })
                .block(selectable_block(AppBlock::RequestContent, app).title("Type"))
                .style(Style::default().fg(Color::Yellow))
                .alignment(Alignment::Center);

                frame.render_widget(content_type_format_p, content_type_chunks[1]);

                let raw_body_input = create_textarea(&app.raw_body, app)
                    .block(selectable_block(AppBlock::RequestContent, app).title("Body"));

                frame.render_widget(raw_body_input, body_chunks[0]);
            } else {
                let rows: Vec<Row> = app
                    .body_form
                    .iter()
                    .map(|(key, value)| {
                        Row::new(vec![key.clone(), value.clone()])
                            .style(Style::default().fg(Color::White))
                    })
                    .collect();

                let table = Table::new(rows)
                    .header(
                        Row::new(vec!["Key", "Value"])
                            .style(Style::default().fg(Color::Yellow))
                            .bottom_margin(1),
                    )
                    .widths(&[Constraint::Percentage(50), Constraint::Percentage(50)])
                    .highlight_style(Style::default().fg(Color::Green))
                    .highlight_symbol(">> ")
                    .block(
                        selectable_block(AppBlock::RequestContent, app)
                            .title("Body")
                            .padding(ratatui::widgets::Padding::new(1, 1, 1, 1)),
                    );

                let mut state = TableState::default();

                state.select(Some(app.selected_form_field.into()));

                frame.render_stateful_widget(table, body_chunks[0], &mut state);
            }
        }
        RequestTab::Headers => {
            let rows: Vec<Row> = app
                .headers
                .iter()
                .map(|(key, value)| {
                    Row::new(vec![key.clone(), value.clone()])
                        .style(Style::default().fg(Color::White))
                })
                .collect();

            let table = Table::new(rows)
                .header(
                    Row::new(vec!["Key", "Value"])
                        .style(Style::default().fg(Color::Yellow))
                        .bottom_margin(1),
                )
                .widths(&[Constraint::Percentage(50), Constraint::Percentage(50)])
                .highlight_style(Style::default().fg(Color::Green))
                .highlight_symbol(">> ")
                .block(
                    selectable_block(AppBlock::RequestContent, app)
                        .title("Headers")
                        .padding(ratatui::widgets::Padding::new(1, 1, 1, 1)),
                );

            let mut state = TableState::default();

            state.select(Some(app.selected_header.into()));

            frame.render_stateful_widget(table, request_chunks[1], &mut state);
        }
        RequestTab::Query => {
            let rows: Vec<Row> = app
                .query_params
                .iter()
                .map(|(key, value)| {
                    Row::new(vec![key.clone(), value.clone()])
                        .style(Style::default().fg(Color::White))
                })
                .collect();

            let table = Table::new(rows)
                .header(
                    Row::new(vec!["Key", "Value"])
                        .style(Style::default().fg(Color::Yellow))
                        .bottom_margin(1),
                )
                .widths(&[Constraint::Percentage(50), Constraint::Percentage(50)])
                .highlight_style(Style::default().fg(Color::Green))
                .highlight_symbol(">> ")
                .block(
                    selectable_block(AppBlock::RequestContent, app)
                        .title("Query Parameters")
                        .padding(ratatui::widgets::Padding::new(1, 1, 1, 1)),
                );

            let mut state = TableState::default();

            state.select(Some(app.selected_query_param.into()));

            frame.render_stateful_widget(table, request_chunks[1], &mut state);
        }
        _ => {}
    }
}
