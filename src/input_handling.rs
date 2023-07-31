use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{App, InputMode};

pub fn handle_input(app: &mut App, key: KeyEvent) {
    match app.input_mode {
        InputMode::Normal => match key.code {
            KeyCode::Char('i') => app.input_mode = InputMode::Insert,
            _ => {}
        },
        InputMode::Insert => match key.code {
            KeyCode::Char(c) => app.endpoint.push(c),
            KeyCode::Backspace => {
                app.endpoint.pop();
            }
            KeyCode::Esc => app.input_mode = InputMode::Normal,
            _ => {}
        },
    }
}
