use crate::event::input::Input;

use super::Navigation;

#[derive(Clone)]
pub struct FormField {
    pub label: String,
    pub hidden: bool,
    pub name: String,
    pub input: Input,
}

impl FormField {
    pub fn new(label: &str, name: &str) -> Self {
        Self {
            label: label.to_owned(),
            hidden: false,
            name: name.to_owned(),
            input: Input::default(),
        }
    }

    pub fn value(mut self, value: &str) -> Self {
        self.input.text = value.to_owned();

        self
    }

    pub fn hidden(mut self) -> Self {
        self.hidden = true;

        self
    }
}

#[derive(Clone)]
pub enum FormKind {
    AddHeader,
    EditHeader,
}

#[derive(Clone)]
pub struct Form {
    pub title: String,

    pub fields: Vec<FormField>,

    pub kind: FormKind,

    pub selected_field: u16,
}

impl Form {
    pub fn new(kind: FormKind, fields: Vec<FormField>) -> Self {
        Self {
            title: String::new(),
            fields,
            kind,
            selected_field: 0,
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_owned();

        self
    }
}

impl Navigation for Form {
    fn next(&mut self) {
        if self.fields.len() - 1 == self.selected_field as usize {
            self.selected_field = 0;
            return;
        }

        self.selected_field += 1;
    }

    fn previous(&mut self) {
        if self.selected_field == 0 {
            self.selected_field = self.fields.len() as u16 - 1;
            return;
        }

        self.selected_field -= 1;
    }
}
