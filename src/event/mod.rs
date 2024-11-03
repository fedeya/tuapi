pub mod input;
mod navigation;
mod popup;

use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{
    form::{Form, FormField, FormKind},
    App, AppBlock, AppPopup, BodyContentType, BodyType, InputMode, OrderNavigation, Request,
    RequestTab,
};

pub async fn handle_input(app: &mut App, key: KeyEvent) {
    if let Some(_) = &mut app.popup {
        popup::handle_popup_events(app, key);
        return;
    }

    match app.input_mode {
        InputMode::Normal => match key.code {
            KeyCode::Char('i') => match app.selected_block {
                AppBlock::Endpoint => {
                    app.input_mode = InputMode::Insert;
                    app.endpoint.move_cursor_to_end_single_line();
                }
                AppBlock::RequestContent => {
                    if let RequestTab::Body = app.request_tab {
                        if let BodyContentType::Text(_) = app.body_content_type {
                            app.input_mode = InputMode::Insert;
                        }
                    }
                }
                _ => {}
            },
            KeyCode::Tab => {
                app.selected_block = app.selected_block.next();
            }
            KeyCode::BackTab => {
                app.selected_block = app.selected_block.previous();
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
                    app.request_tab = app.request_tab.previous();
                }
                AppBlock::RequestContent => match app.request_tab {
                    RequestTab::Headers => {
                        let quantity = app.headers.len() as u16;

                        if quantity == 0 {
                            app.selected_query_param = 0;
                            return;
                        }

                        if app.selected_header < quantity - 1 {
                            app.selected_header += 1;
                        } else {
                            app.selected_header = 0;
                        }
                    }

                    RequestTab::Query => {
                        let quantity = app.query_params.len() as u16;

                        if quantity == 0 {
                            app.selected_query_param = 0;
                            return;
                        }

                        if app.selected_query_param < quantity - 1 {
                            app.selected_query_param += 1;
                        } else {
                            app.selected_query_param = 0;
                        }
                    }

                    RequestTab::Body => {
                        if let BodyContentType::Form = app.body_content_type {
                            let quantity = app.body_form.len() as u16;

                            if quantity == 0 {
                                app.selected_form_field = 0;
                                return;
                            }

                            if app.selected_form_field < quantity - 1 {
                                app.selected_form_field += 1;
                            } else {
                                app.selected_form_field = 0;
                            }
                        }
                    }
                    _ => {}
                },
                AppBlock::Method => app.method = app.method.previous(),
                _ => {}
            },
            KeyCode::Char('k') => match app.selected_block {
                AppBlock::Response => {
                    navigation::scroll_up_response(app);
                }
                AppBlock::Request => app.request_tab = app.request_tab.next(),
                AppBlock::RequestContent => match app.request_tab {
                    RequestTab::Headers => {
                        let quantity = app.headers.len() as u16;

                        if quantity == 0 {
                            app.selected_header = 0;
                            return;
                        }

                        if app.selected_header > 0 {
                            app.selected_header -= 1;
                        } else {
                            app.selected_header = quantity - 1;
                        }
                    }
                    RequestTab::Query => {
                        let quantity = app.query_params.len() as u16;

                        if quantity == 0 {
                            app.selected_query_param = 0;
                            return;
                        }

                        if app.selected_query_param > 0 {
                            app.selected_query_param -= 1;
                        } else {
                            app.selected_query_param = quantity - 1;
                        }
                    }

                    RequestTab::Body => {
                        if let BodyContentType::Form = app.body_content_type {
                            let quantity = app.body_form.len() as u16;

                            if quantity == 0 {
                                app.selected_form_field = 0;
                                return;
                            }

                            if app.selected_form_field > 0 {
                                app.selected_form_field -= 1;
                            } else {
                                app.selected_form_field = quantity - 1;
                            }
                        }
                    }
                    _ => {}
                },
                AppBlock::Method => app.method = app.method.next(),
                _ => {}
            },
            KeyCode::Char('c') => match app.selected_block {
                AppBlock::RequestContent => match app.request_tab {
                    RequestTab::Body => {
                        app.body_content_type = match app.body_content_type {
                            BodyContentType::Text(_) => BodyContentType::Form,
                            BodyContentType::Form => BodyContentType::Text(BodyType::Raw),
                        };
                    }
                    _ => {}
                },
                _ => {}
            },
            KeyCode::Char('t') => match app.selected_block {
                AppBlock::RequestContent => match app.request_tab {
                    RequestTab::Body => {
                        if let BodyContentType::Text(body_type) = app.body_content_type.clone() {
                            let new_body_type = match body_type {
                                BodyType::Raw => BodyType::Json,
                                BodyType::Json => BodyType::Xml,
                                BodyType::Xml => BodyType::Raw,
                            };

                            match new_body_type {
                                BodyType::Json => {
                                    app.headers.insert(
                                        "Content-Type".to_owned(),
                                        "application/json".to_owned(),
                                    );
                                }
                                BodyType::Raw => {
                                    app.headers
                                        .insert("Content-Type".to_owned(), "text/plain".to_owned());
                                }
                                BodyType::Xml => {
                                    app.headers.insert(
                                        "Content-Type".to_owned(),
                                        "application/xml".to_owned(),
                                    );
                                }
                            }

                            app.body_content_type = BodyContentType::Text(new_body_type);
                        }
                    }
                    _ => {}
                },
                _ => {}
            },
            KeyCode::Char('a') => match app.selected_block {
                AppBlock::RequestContent => match app.request_tab {
                    RequestTab::Headers => {
                        let key_input = FormField::new("Key", "key");

                        let value_input = FormField::new("Value", "value");

                        let form = Form::new(FormKind::AddHeader, vec![key_input, value_input])
                            .title("Add Header");

                        app.popup = Some(AppPopup::FormPopup(form));
                    }
                    RequestTab::Query => {
                        let key_input = FormField::new("Key", "key");

                        let value_input = FormField::new("Value", "value");

                        let form = Form::new(FormKind::AddQueryParam, vec![key_input, value_input])
                            .title("Add Query Param");

                        app.popup = Some(AppPopup::FormPopup(form));
                    }
                    RequestTab::Body => {
                        if let BodyContentType::Form = app.body_content_type {
                            let key_input = FormField::new("Key", "key");

                            let value_input = FormField::new("Value", "value");

                            let form =
                                Form::new(FormKind::AddBodyFormField, vec![key_input, value_input])
                                    .title("Add Form Field");

                            app.popup = Some(AppPopup::FormPopup(form));
                        }
                    }
                    _ => {}
                },
                _ => {}
            },
            KeyCode::Char('e') => match app.selected_block {
                AppBlock::RequestContent => match app.request_tab {
                    RequestTab::Headers => {
                        let key = app
                            .headers
                            .clone()
                            .keys()
                            .nth(app.selected_header as usize)
                            .unwrap()
                            .to_owned();

                        let value = app.headers.get(&key).unwrap().to_owned();

                        let key_input = FormField::new("Key", "key").value(&key);

                        let current_key = FormField::new("Current Key", "current_key")
                            .value(&key)
                            .hidden();

                        let value_input = FormField::new("Value", "value").value(&value);

                        let form = Form::new(
                            FormKind::EditHeader,
                            vec![key_input, value_input, current_key],
                        )
                        .title("Edit Header");

                        app.popup = Some(AppPopup::FormPopup(form));
                    }
                    RequestTab::Query => {
                        let (key, value) =
                            app.query_params[app.selected_query_param as usize].clone();

                        let key_input = FormField::new("Key", "key").value(&key);

                        let value_input = FormField::new("Value", "value").value(&value);

                        let index_input = FormField::new("Index", "index")
                            .value(&app.selected_query_param.to_string())
                            .hidden();

                        let form = Form::new(
                            FormKind::EditQueryParam,
                            vec![key_input, value_input, index_input],
                        )
                        .title("Edit Query Param");

                        app.popup = Some(AppPopup::FormPopup(form));
                    }
                    RequestTab::Body => {
                        if let BodyContentType::Form = app.body_content_type {
                            let (key, value) = app
                                .body_form
                                .iter()
                                .nth(app.selected_form_field as usize)
                                .unwrap();

                            let key_input = FormField::new("Key", "key").value(key);

                            let value_input = FormField::new("Value", "value").value(value);

                            let current_key = FormField::new("Current Key", "current_key")
                                .value(&key)
                                .hidden();

                            let form = Form::new(
                                FormKind::EditBodyFormField,
                                vec![key_input, value_input, current_key],
                            )
                            .title("Edit Form Field");

                            app.popup = Some(AppPopup::FormPopup(form));
                        }
                    }
                    _ => {}
                },
                _ => {}
            },
            KeyCode::Char('d') => match app.selected_block {
                AppBlock::RequestContent => match app.request_tab {
                    RequestTab::Headers => {
                        if app.headers.len() == 0 {
                            return;
                        }

                        let key = app
                            .headers
                            .clone()
                            .keys()
                            .nth(app.selected_header as usize)
                            .unwrap()
                            .to_owned();

                        app.headers.remove(&key);

                        if app.selected_header as usize == app.headers.len()
                            && app.headers.len() != 0
                        {
                            app.selected_header -= 1;
                        }
                    }
                    RequestTab::Query => {
                        if app.query_params.len() == 0 {
                            return;
                        }

                        app.query_params.remove(app.selected_query_param as usize);

                        if app.selected_query_param as usize == app.query_params.len()
                            && app.query_params.len() != 0
                        {
                            app.selected_query_param -= 1;
                        }
                    }
                    RequestTab::Body => {
                        if let BodyContentType::Form = app.body_content_type {
                            if app.body_form.len() == 0 {
                                return;
                            }

                            let key = app
                                .body_form
                                .clone()
                                .keys()
                                .nth(app.selected_form_field as usize)
                                .unwrap()
                                .to_owned();

                            app.body_form.remove(&key);

                            if app.selected_header as usize == app.body_form.len()
                                && app.body_form.len() != 0
                            {
                                app.selected_form_field -= 1;
                            }
                        }
                    }
                    _ => {}
                },
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
                    if let RequestTab::Body = app.request_tab {
                        app.raw_body.add_char_at_cursor(c);
                    }
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
