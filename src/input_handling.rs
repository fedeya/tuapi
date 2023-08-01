use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{App, AppBlock, InputMode},
    request,
};

pub fn handle_input(app: &mut App, key: KeyEvent) {
    match app.input_mode {
        InputMode::Normal => match key.code {
            KeyCode::Char('i') => match app.selected_block {
                AppBlock::Endpoint => {
                    app.input_mode = InputMode::Insert;
                    app.input_cursor_position = app.endpoint.len().try_into().unwrap()
                }
                AppBlock::Request => {
                    app.input_mode = InputMode::Insert;
                }
                _ => {}
            },
            KeyCode::Char('n') => {
                if let AppBlock::Request = app.selected_block {
                    app.request_tab = if app.request_tab >= 4 {
                        0
                    } else {
                        app.request_tab + 1
                    };
                }
            }
            KeyCode::Tab => {
                let mut selected_block: u16 = app.selected_block.clone().into();

                selected_block += 1;

                if selected_block > 4 {
                    selected_block = 1;
                }

                app.selected_block = selected_block.into();
            }
            KeyCode::BackTab => {
                let mut selected_block: u16 = app.selected_block.clone().into();

                selected_block -= 1;

                if selected_block == 0 {
                    selected_block = 4;
                }

                app.selected_block = selected_block.into();
            }
            KeyCode::Enter => {
                request::handle_request(app);
            }

            KeyCode::Char('j') => {
                if let AppBlock::Response = app.selected_block {
                    let x = app.response_scroll.0 + 2;

                    app.response_scroll.0 = x;
                }
            }
            KeyCode::Char('k') => {
                if let AppBlock::Response = app.selected_block {
                    let x = if app.response_scroll.0 == 0 {
                        0
                    } else {
                        if app.response_scroll.0 - 2 > 0 {
                            app.response_scroll.0 - 2
                        } else {
                            0
                        }
                    };

                    app.response_scroll.0 = x;
                }
            }
            _ => {}
        },
        InputMode::Insert => match key.code {
            KeyCode::Char(c) => match app.selected_block {
                AppBlock::Endpoint => {
                    app.endpoint.insert(app.input_cursor_position.into(), c);
                    app.input_cursor_position += 1;
                }
                AppBlock::Request => {
                    app.raw_body.push(c);
                }
                _ => {}
            },
            KeyCode::Right => {
                if let AppBlock::Endpoint = app.selected_block {
                    let new_pos = app.input_cursor_position + 1;

                    app.input_cursor_position =
                        new_pos.clamp(0, app.endpoint.chars().count().try_into().unwrap());
                }
            }
            KeyCode::Left => {
                if let AppBlock::Endpoint = app.selected_block {
                    let new_pos = if app.input_cursor_position == 0 {
                        0
                    } else {
                        app.input_cursor_position - 1
                    };

                    app.input_cursor_position =
                        new_pos.clamp(0, app.endpoint.chars().count().try_into().unwrap());
                }
            }
            KeyCode::Enter => match app.selected_block {
                AppBlock::Request => {
                    app.raw_body.push('\n');
                }
                AppBlock::Endpoint => {
                    request::handle_request(app);

                    app.input_mode = InputMode::Normal;
                }
                _ => {}
            },
            KeyCode::Backspace => match app.selected_block {
                AppBlock::Endpoint => {
                    let removable = app.input_cursor_position != 0;

                    if removable {
                        app.endpoint.remove(app.input_cursor_position as usize - 1);
                        app.input_cursor_position -= 1;
                    }
                }
                AppBlock::Request => {
                    app.raw_body.pop();
                }
                _ => {}
            },
            KeyCode::Esc => app.input_mode = InputMode::Normal,
            _ => {}
        },
    }
}
