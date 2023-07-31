#[derive(Debug, PartialEq)]
pub enum InputMode {
    Normal,
    Insert,
}

#[derive(Debug, Copy, Clone)]
pub enum RequestMethod {
    Get,
    Post,
    Put,
    Delete,
}

impl ToString for RequestMethod {
    fn to_string(&self) -> String {
        match self {
            Self::Get => "GET".to_string(),
            Self::Post => "POST".to_string(),
            Self::Put => "PUT".to_string(),
            Self::Delete => "DELETE".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct App {
    pub input_mode: InputMode,

    pub endpoint: String,
    pub method: RequestMethod,

    pub raw_body: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            input_mode: InputMode::Normal,
            endpoint: String::new(),
            raw_body: String::new(),
            method: RequestMethod::Delete,
        }
    }
}
