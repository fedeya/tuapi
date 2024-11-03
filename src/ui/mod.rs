mod input;
mod syntax;

use std::{io::Stdout, iter::once};

use ratatui::{
    prelude::{Alignment, Constraint, CrosstermBackend, Direction, Layout, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Clear, Paragraph, Row, Table, TableState, Tabs},
    Frame,
};

use crate::app::{App, AppBlock, AppPopup, InputMode, OrderNavigation, RequestMethod, RequestTab};

use self::input::{create_input, create_textarea};

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

    let raw_body_input = create_textarea(&app.raw_body, app)
        .block(selectable_block(AppBlock::RequestContent, app).title("Body"));

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
        Span::styled("Auth", Style::default().fg(Color::White)),
        Span::styled("Cookies", Style::default().fg(Color::White)),
    ];

    let tab = Tabs::new(request_tabs)
        .block(selectable_block(AppBlock::Request, app))
        .divider(Span::raw("|"))
        .select(app.request_tab.clone().get_index())
        .highlight_style(Style::default().fg(Color::Green));

    frame.render_widget(method_p, header_chunks[0]);
    frame.render_widget(endpoint_input, header_chunks[1]);

    frame.render_widget(tab, request_chunks[0]);

    match app.request_tab {
        RequestTab::Body => {
            frame.render_widget(raw_body_input, request_chunks[1]);
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
        _ => {}
    }

    match app.response.as_ref() {
        Some(r) => {
            let lines_count = u16::try_from(r.text.lines().count()).unwrap_or(1);
            let max_x = if lines_count > response_chunks[0].height {
                lines_count - (response_chunks[0].height - 2)
            } else {
                0
            };

            app.response_scroll.0 = app.response_scroll.0.clamp(0, max_x);

            let lines = syntax::highlight_response(r.text.clone());

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

    frame.render_widget(help_p, main_chunks[2]);

    match app.popup.as_ref() {
        Some(AppPopup::ChangeMethod) => {
            let block = Block::default()
                .title("Select method")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White));

            let methods = app.method.get_order();

            let methods_blocks = methods.iter().enumerate().map(|(i, method)| {
                let cloned_method = method.clone();

                let border_style = match cloned_method == app.method.clone() {
                    true => Style::default().fg(Color::Green),
                    false => Style::default().fg(Color::White),
                };

                let style = Style::default().fg(match cloned_method {
                    RequestMethod::Get => Color::Green,
                    RequestMethod::Post => Color::Blue,
                    RequestMethod::Put => Color::Yellow,
                    RequestMethod::Delete => Color::Red,
                    RequestMethod::Patch => Color::Magenta,
                });

                let block = Paragraph::new(cloned_method.to_string())
                    .style(style)
                    .alignment(Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(border_style),
                    );

                (i, block)
            });

            let height = app.method.get_order().len() as u16 * 3;

            let width = 40;

            let area = centered_rect(width, height + 4, frame.size());

            frame.render_widget(Clear, area);
            frame.render_widget(block, area);

            methods_blocks.for_each(|(index, p)| {
                frame.render_widget(
                    p,
                    Rect::new(area.x + 2, area.y + index as u16 * 3 + 1, width - 4, 3),
                );
            });

            let help_p = Paragraph::new("Use j/k to navigate, Enter to select")
                .style(Style::default().fg(Color::White))
                .alignment(Alignment::Center);

            frame.render_widget(
                help_p,
                Rect::new(area.x + 2, area.y + height + 2, width - 4, 1),
            );
        }
        Some(AppPopup::FormPopup(form)) => {
            let block = Block::default()
                .title(form.title.clone())
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue));

            let visible_fields = form.visible_fields();

            let height = visible_fields.len() * 3 + 4;

            let area = centered_rect(70, height as u16, frame.size());

            let inputs = visible_fields.iter().enumerate().map(|(index, field)| {
                let input = create_input(&field.input, &app, index == form.selected_field as usize)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(
                                if index == form.selected_field
                                    && app.input_mode == InputMode::Insert
                                {
                                    Color::Green
                                } else if index == form.selected_field {
                                    Color::Blue
                                } else {
                                    Color::White
                                },
                            ))
                            .title(field.label.clone()),
                    );

                (index, input)
            });

            frame.render_widget(Clear, area);
            frame.render_widget(block, area);

            inputs.for_each(|(index, p)| {
                frame.render_widget(
                    p,
                    Rect::new(area.x + 2, area.y + index as u16 * 3 + 1, area.width - 4, 3),
                );
            });

            frame.render_widget(
                Paragraph::new("Press Enter to Accept Changes")
                    .style(Style::default().fg(Color::White))
                    .alignment(Alignment::Center),
                Rect::new(area.x + 2, area.y + height as u16 - 2, area.width - 4, 1),
            );
        }
        None => {}
    }
}

fn centered_rect(width: u16, height: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length((r.height - height) / 2),
                Constraint::Length(height),
                Constraint::Length((r.height - height) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length((r.width - width) / 2),
                Constraint::Length(width),
                Constraint::Length((r.width - width) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
