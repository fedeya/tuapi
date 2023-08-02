use std::collections::HashMap;

#[derive(PartialEq)]
pub enum InputMode {
    Normal,
    Insert,
}

#[derive(Clone)]
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

#[derive(Clone, PartialEq)]
pub enum AppBlock {
    Endpoint,
    Method,
    Request,
    RequestContent,
    Response,
}

#[derive(Clone)]
pub enum RequestTab {
    Body,
    Query,
    Headers,
    Auth,
    Cookies,
}

impl From<RequestTab> for usize {
    fn from(tab: RequestTab) -> Self {
        match tab {
            RequestTab::Body => 0,
            RequestTab::Query => 1,
            RequestTab::Headers => 2,
            RequestTab::Auth => 3,
            RequestTab::Cookies => 4,
        }
    }
}

impl Into<RequestTab> for usize {
    fn into(self) -> RequestTab {
        match self {
            0 => RequestTab::Body,
            1 => RequestTab::Query,
            2 => RequestTab::Headers,
            3 => RequestTab::Auth,
            4 => RequestTab::Cookies,
            _ => panic!("Invalid tab index"),
        }
    }
}

impl From<AppBlock> for u16 {
    fn from(block: AppBlock) -> Self {
        match block {
            AppBlock::Method => 1,
            AppBlock::Endpoint => 2,
            AppBlock::Request => 3,
            AppBlock::RequestContent => 4,
            AppBlock::Response => 5,
        }
    }
}

impl Into<AppBlock> for u16 {
    fn into(self) -> AppBlock {
        match self {
            1 => AppBlock::Method,
            2 => AppBlock::Endpoint,
            3 => AppBlock::Request,
            4 => AppBlock::RequestContent,
            5 => AppBlock::Response,
            _ => panic!("Invalid block index"),
        }
    }
}

pub struct Response {
    pub status_code: u16,
    pub text: String,
}

pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}

impl Default for Coordinates {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

pub struct Input {
    pub text: String,
    pub cursor_position: Coordinates,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            text: String::from(" "),
            cursor_position: Coordinates::default(),
        }
    }
}

pub struct App {
    pub input_mode: InputMode,

    pub endpoint: Input,
    pub method: RequestMethod,

    pub raw_body: Input,
    pub request_tab: RequestTab,

    pub selected_block: AppBlock,

    pub response: Option<Response>,
    pub headers: HashMap<String, String>,
    pub response_scroll: (u16, u16),
}

impl Default for App {
    fn default() -> Self {
        let mut headers = HashMap::new();

        headers.insert("Content-Type".to_string(), "application/json".to_string());

        Self {
            input_mode: InputMode::Normal,
            endpoint: Input {
                text: String::from("https://jsonplaceholder.typicode.com/users"),
                ..Input::default()
            },
            headers,
            raw_body: Input::default(),
            method: RequestMethod::Get,
            request_tab: RequestTab::Body,
            selected_block: AppBlock::Endpoint,
            response: None,
            response_scroll: (0, 0),
        }
    }
}
