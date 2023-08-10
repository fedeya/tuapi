pub mod input;
mod navigation;

use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{App, AppBlock, AppPopup, InputMode, Request, RequestMethod};

pub async fn handle_input(app: &mut App, key: KeyEvent) {
    match app.input_mode {
        InputMode::Normal => match key.code {
            KeyCode::Char('i') => match app.selected_block {
                AppBlock::Endpoint => {
                    app.input_mode = InputMode::Insert;
                    app.endpoint.move_cursor_to_end_single_line();
                }
                AppBlock::RequestContent => {
                    app.input_mode = InputMode::Insert;
                }
                _ => {}
            },
            KeyCode::Tab => {
                app.selected_block.next();
            }
            KeyCode::BackTab => {
                app.selected_block.previous();
            }
            KeyCode::Enter => match app.selected_block {
                AppBlock::Request => {
                    app.selected_block = AppBlock::RequestContent;
                }
                AppBlock::Method => {
                    app.popup = Some(AppPopup::ChangeMethod);
                }
                _ => {
                    app.is_loading = true;
                    app.req_tx.send(Request::from_app(&app)).await.unwrap();
                }
            },

            KeyCode::Char('j') => match app.selected_block {
                AppBlock::Response => {
                    navigation::scroll_down_response(app);
                }
                AppBlock::Request => {
                    app.request_tab.previous();
                }
                AppBlock::Method => {
                    app.method = RequestMethod::Get;
                }
                _ => {}
            },
            KeyCode::Char('k') => match app.selected_block {
                AppBlock::Response => {
                    navigation::scroll_up_response(app);
                }
                AppBlock::Request => {
                    app.request_tab.next();
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
                    app.endpoint.add_char_at_cursor(c);
                }
                AppBlock::RequestContent => {
                    app.raw_body.add_char_at_cursor(c);
                }
                _ => {}
            },
            KeyCode::Up => match app.selected_block {
                AppBlock::RequestContent => {
                    app.raw_body.move_cursor_up();
                }
                _ => {}
            },
            KeyCode::Down => match app.selected_block {
                AppBlock::RequestContent => {
                    app.raw_body.move_cursor_down();
                }
                _ => {}
            },
            KeyCode::Right => match app.selected_block {
                AppBlock::Endpoint => {
                    app.endpoint.move_cursor_right();
                }
                AppBlock::RequestContent => {
                    app.raw_body.move_cursor_right();
                }
                _ => (),
            },
            KeyCode::Left => match app.selected_block {
                AppBlock::Endpoint => {
                    app.endpoint.move_cursor_left();
                }
                AppBlock::RequestContent => {
                    app.raw_body.move_cursor_left();
                }
                _ => (),
            },
            KeyCode::Enter => match app.selected_block {
                AppBlock::RequestContent => {
                    app.raw_body.add_newline_at_cursor();
                }
                AppBlock::Endpoint => {
                    app.is_loading = true;

                    app.req_tx.send(Request::from_app(&app)).await.unwrap();

                    app.input_mode = InputMode::Normal;
                }
                _ => {}
            },
            KeyCode::Tab => {
                if let AppBlock::RequestContent = app.selected_block {
                    app.raw_body.add_char_at_cursor(' ');
                    app.raw_body.add_char_at_cursor(' ');
                }
            }
            KeyCode::Backspace => match app.selected_block {
                AppBlock::Endpoint => {
                    app.endpoint.remove_char_before_cursor_single_line();
                }
                AppBlock::RequestContent => {
                    app.raw_body.remove_char_before_cursor();
                }
                _ => {}
            },
            KeyCode::Esc => app.input_mode = InputMode::Normal,
            _ => {}
        },
    }
}
