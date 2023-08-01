mod input;
mod navigation;

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
                    input::move_cursor_to_end_of_line(&mut app.endpoint);
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
                navigation::move_to_next_block(app);
            }
            KeyCode::BackTab => {
                navigation::move_to_previous_block(app);
            }
            KeyCode::Enter => {
                request::handle_request(app);
            }

            KeyCode::Char('j') => {
                navigation::scroll_up_response(app);
            }
            KeyCode::Char('k') => {
                navigation::scroll_down_response(app);
            }
            _ => {}
        },
        InputMode::Insert => match key.code {
            KeyCode::Char(c) => match app.selected_block {
                AppBlock::Endpoint => {
                    input::add_char_at_cursor(&mut app.endpoint, c);
                }
                AppBlock::Request => {
                    app.raw_body.push(c);
                }
                _ => {}
            },
            KeyCode::Right => {
                if let AppBlock::Endpoint = app.selected_block {
                    input::move_cursor_right(&mut app.endpoint);
                }
            }
            KeyCode::Left => {
                if let AppBlock::Endpoint = app.selected_block {
                    input::move_cursor_left(&mut app.endpoint);
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
                    input::remove_char_before_cursor(&mut app.endpoint);
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
