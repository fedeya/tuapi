use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{App, AppBlock, InputMode},
    request,
};

pub fn handle_input(app: &mut App, key: KeyEvent) {
    match app.input_mode {
        InputMode::Normal => match key.code {
            KeyCode::Char('i') => app.input_mode = InputMode::Insert,
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
                    app.endpoint.push(c);
                }
                AppBlock::Request => {
                    app.raw_body.push(c);
                }
                _ => {}
            },
            KeyCode::Enter => {
                if let AppBlock::Request = app.selected_block {
                    app.raw_body.push('\n');
                }
            }
            KeyCode::Backspace => match app.selected_block {
                AppBlock::Endpoint => {
                    app.endpoint.pop();
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
