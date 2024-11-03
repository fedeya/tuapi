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

    /// Set the value of the input
    pub fn value(mut self, value: &str) -> Self {
        self.input.text = value.to_owned();

        self
    }

    /// Set the input to be hidden
    pub fn hidden(mut self) -> Self {
        self.hidden = true;

        self
    }
}

#[derive(Clone)]
pub enum FormKind {
    AddHeader,
    EditHeader,
    AddQueryParam,
    EditQueryParam,
}

#[derive(Clone)]
pub struct Form {
    pub title: String,

    pub fields: Vec<FormField>,

    pub kind: FormKind,

    pub selected_field: usize,
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

    /// Set the title of the form
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_owned();

        self
    }
}

impl Form {
    /// Returns all fields that are not hidden
    pub fn visible_fields(&self) -> Vec<FormField> {
        self.fields
            .iter()
            .filter(|field| !field.hidden)
            .cloned()
            .collect()
    }
}

impl Navigation for Form {
    /// Go to the next field
    fn next(&mut self) {
        if self.visible_fields().len() - 1 == self.selected_field {
            self.selected_field = 0;
            return;
        }

        self.selected_field += 1;
    }

    /// Go to the previous field
    fn previous(&mut self) {
        if self.selected_field == 0 {
            self.selected_field = self.visible_fields().len() - 1;
            return;
        }

        self.selected_field -= 1;
    }
}
