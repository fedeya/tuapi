use std::io::Stdout;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::CrosstermBackend,
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::{App, AppPopup, InputMode, OrderNavigation, RequestMethod};

use super::input::create_input;

pub fn render_popup(app: &App, frame: &mut Frame<'_, CrosstermBackend<Stdout>>) {
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
