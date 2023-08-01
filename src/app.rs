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
    Response,
}

impl From<AppBlock> for u16 {
    fn from(block: AppBlock) -> Self {
        match block {
            AppBlock::Method => 1,
            AppBlock::Endpoint => 2,
            AppBlock::Request => 3,
            AppBlock::Response => 4,
        }
    }
}

impl Into<AppBlock> for u16 {
    fn into(self) -> AppBlock {
        match self {
            1 => AppBlock::Method,
            2 => AppBlock::Endpoint,
            3 => AppBlock::Request,
            4 => AppBlock::Response,
            _ => panic!("Invalid block index"),
        }
    }
}

pub struct Response {
    pub status_code: u16,
    pub text: String,
}

pub struct App {
    pub input_mode: InputMode,
    pub input_cursor_position: u16,

    pub endpoint: String,
    pub method: RequestMethod,

    pub raw_body: String,
    pub request_tab: u8,

    pub selected_block: AppBlock,

    pub response: Option<Response>,
    pub response_scroll: (u16, u16),
}

impl Default for App {
    fn default() -> Self {
        Self {
            input_mode: InputMode::Normal,
            endpoint: String::from("https://jsonplaceholder.typicode.com/users"),
            raw_body: String::new(),
            method: RequestMethod::Get,
            request_tab: 0,
            input_cursor_position: 0,
            selected_block: AppBlock::Endpoint,
            response: None,
            response_scroll: (0, 0),
        }
    }
}
