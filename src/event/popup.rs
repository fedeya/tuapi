use crate::app::{
    form::{Form, FormKind},
    App, AppPopup, InputMode, Navigation, OrderNavigation,
};
use crossterm::event::{KeyCode, KeyEvent};

use std::collections::HashMap;

pub fn handle_popup_events(app: &mut App, key: KeyEvent) {
    match app.popup.as_mut().unwrap() {
        AppPopup::FormPopup(f) => match app.input_mode {
            InputMode::Normal => match key.code {
                KeyCode::Char('i') => {
                    app.input_mode = InputMode::Insert;

                    f.fields
                        .get_mut(f.selected_field as usize)
                        .unwrap()
                        .input
                        .move_cursor_to_end_single_line();
                }
                KeyCode::Char('j') => f.next(),
                KeyCode::Char('k') => f.previous(),
                KeyCode::Enter => {
                    handle_forms(f.clone(), app);
                    app.popup = None;
                }
                _ => {}
            },
            InputMode::Insert => match key.code {
                KeyCode::Esc => {
                    app.input_mode = InputMode::Normal;
                }
                KeyCode::Enter => {
                    handle_forms(f.clone(), app);
                    app.input_mode = InputMode::Normal;
                    app.popup = None;
                }
                KeyCode::BackTab => {
                    f.previous();

                    f.fields
                        .get_mut(f.selected_field as usize)
                        .unwrap()
                        .input
                        .move_cursor_to_end_single_line()
                }
                KeyCode::Tab => {
                    f.next();
                    f.fields
                        .get_mut(f.selected_field as usize)
                        .unwrap()
                        .input
                        .move_cursor_to_end_single_line()
                }
                KeyCode::Left => {
                    f.fields
                        .get_mut(f.selected_field as usize)
                        .unwrap()
                        .input
                        .move_cursor_left();
                }
                KeyCode::Right => {
                    f.fields
                        .get_mut(f.selected_field as usize)
                        .unwrap()
                        .input
                        .move_cursor_right();
                }
                KeyCode::Backspace => {
                    f.fields
                        .get_mut(f.selected_field as usize)
                        .unwrap()
                        .input
                        .remove_char_before_cursor_single_line();
                }
                KeyCode::Char(c) => {
                    f.fields
                        .get_mut(f.selected_field as usize)
                        .unwrap()
                        .input
                        .add_char_at_cursor(c);
                }
                _ => {}
            },
        },

        AppPopup::ChangeMethod => match key.code {
            KeyCode::Char('k') => app.method = app.method.previous(),
            KeyCode::Char('j') => app.method = app.method.next(),
            KeyCode::Enter => {
                app.popup = None;
            }
            KeyCode::Esc => app.popup = None,
            _ => {}
        },
        _ => {}
    }
}

fn handle_forms(form: Form, app: &mut App) {
    let values = form
        .fields
        .iter()
        .map(|field| (field.name.clone(), field.input.text.clone()))
        .collect::<HashMap<String, String>>();

    match form.kind {
        FormKind::EditHeader => {
            let current_key = values.get("current_key").unwrap().to_owned();
            let key = values.get("key").unwrap().to_owned();
            let value = values.get("value").unwrap().to_owned();

            if current_key == key {
                app.headers.insert(key, value);
            } else {
                app.headers.remove(&current_key);

                app.headers.insert(key, value);
            }
        }
        FormKind::AddHeader => {
            let key = values.get("key").unwrap().to_owned();
            let value = values.get("value").unwrap().to_owned();

            app.headers.insert(key, value);
        }

        FormKind::AddQueryParam => {
            let key = values.get("key").unwrap().to_owned();
            let value = values.get("value").unwrap().to_owned();

            app.query_params.push((key, value));
        }

        FormKind::EditQueryParam => {
            let key = values.get("key").unwrap().to_owned();
            let value = values.get("value").unwrap().to_owned();

            app.query_params[app.selected_query_param as usize] = (key, value);
        }
    }
}
