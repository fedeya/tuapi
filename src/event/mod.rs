mod input;
mod navigation;

use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{App, AppBlock, InputMode, RequestMethod},
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
                AppBlock::RequestContent => {
                    app.input_mode = InputMode::Insert;
                }
                _ => {}
            },
            KeyCode::Tab => {
                navigation::move_to_next_block(app);
            }
            KeyCode::BackTab => {
                navigation::move_to_previous_block(app);
            }
            KeyCode::Enter => {
                request::handle_request(app);
            }

            KeyCode::Char('j') => match app.selected_block {
                AppBlock::Response => {
                    navigation::scroll_up_response(app);
                }
                AppBlock::Request => {
                    navigation::move_next_request_tab(app);
                }
                AppBlock::Method => {
                    app.method = RequestMethod::Get;
                }
                _ => {}
            },
            KeyCode::Char('k') => match app.selected_block {
                AppBlock::Response => {
                    navigation::scroll_down_response(app);
                }
                AppBlock::Request => {
                    navigation::move_to_previous_request_tab(app);
                }
                AppBlock::Method => {
                    app.method = RequestMethod::Post;
                }
                _ => {}
            },
            _ => {}
        },
        InputMode::Insert => match key.code {
            KeyCode::Char(c) => match app.selected_block {
                AppBlock::Endpoint => {
                    input::add_char_at_cursor(&mut app.endpoint, c);
                }
                AppBlock::RequestContent => {
                    input::add_char_at_cursor(&mut app.raw_body, c);
                }
                _ => {}
            },
            KeyCode::Up => match app.selected_block {
                AppBlock::RequestContent => {
                    input::move_cursor_up(&mut app.raw_body);
                }
                _ => {}
            },
            KeyCode::Down => match app.selected_block {
                AppBlock::RequestContent => {
                    input::move_cursor_down(&mut app.raw_body);
                }
                _ => {}
            },
            KeyCode::Right => match app.selected_block {
                AppBlock::Endpoint => {
                    input::move_cursor_right(&mut app.endpoint);
                }
                AppBlock::RequestContent => {
                    input::move_cursor_right(&mut app.raw_body);
                }
                _ => (),
            },
            KeyCode::Left => match app.selected_block {
                AppBlock::Endpoint => {
                    input::move_cursor_left(&mut app.endpoint);
                }
                AppBlock::RequestContent => {
                    input::move_cursor_left(&mut app.raw_body);
                }
                _ => (),
            },
            KeyCode::Enter => match app.selected_block {
                AppBlock::RequestContent => {
                    input::add_newline_at_cursor(&mut app.raw_body);
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
                AppBlock::RequestContent => {
                    input::remove_char_before_cursor(&mut app.raw_body);
                }
                _ => {}
            },
            KeyCode::Esc => app.input_mode = InputMode::Normal,
            _ => {}
        },
    }
}
