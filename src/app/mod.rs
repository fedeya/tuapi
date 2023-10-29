pub mod form;

use crate::event::input::Input;
use form::Form;
use std::collections::HashMap;

use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::request;

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

pub trait Navigation {
    /// Go to the next item
    fn next(&mut self) {}

    /// Go to the previous item
    fn previous(&mut self) {}
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

impl Navigation for RequestTab {
    fn next(&mut self) {
        let mut selected_tab: usize = self.clone().into();

        selected_tab += 1;

        if selected_tab > 4 {
            selected_tab = 0;
        }

        *self = selected_tab.into();
    }

    fn previous(&mut self) {
        let mut seleced_tab: usize = self.clone().into();

        if seleced_tab == 0 {
            seleced_tab = 4;
        } else {
            seleced_tab -= 1;
        }

        *self = seleced_tab.into();
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

impl Navigation for AppBlock {
    fn next(&mut self) {
        let mut selected_block: u16 = self.clone().into();

        selected_block += 1;

        if selected_block > 5 {
            selected_block = 1;
        }

        *self = selected_block.into();
    }

    fn previous(&mut self) {
        let mut selected_block: u16 = self.clone().into();

        selected_block -= 1;

        if selected_block == 0 {
            selected_block = 5;
        }

        *self = selected_block.into();
    }
}

#[derive(Debug)]
pub struct Response {
    pub status_code: u16,
    pub text: String,
}

#[derive(Clone)]
pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}

impl Default for Coordinates {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

pub enum AppPopup {
    ChangeMethod,
    FormPopup(Form),
}

pub struct Request {
    pub method: RequestMethod,
    pub endpoint: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Request {
    pub fn from_app(app: &App) -> Self {
        Self {
            method: app.method.clone(),
            endpoint: app.endpoint.text.clone(),
            headers: app.headers.clone(),
            body: app.raw_body.text.clone(),
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

    pub selected_header: u16,

    pub response: Option<Response>,

    pub res_rx: Receiver<Option<Response>>,
    pub req_tx: Sender<Request>,
    pub is_loading: bool,

    pub headers: HashMap<String, String>,
    pub response_scroll: (u16, u16),

    pub popup: Option<AppPopup>,
}

fn handle_requests(mut req_rx: Receiver<Request>, res_tx: Sender<Option<Response>>) {
    tokio::spawn(async move {
        while let Some(req) = req_rx.recv().await {
            let res = request::send(req).await;

            res_tx.send(Some(res)).await.unwrap();
        }
    });
}

impl Default for App {
    fn default() -> Self {
        let headers = HashMap::from([
            ("Content-Type".to_string(), "application/json".to_string()),
            ("Accept".to_string(), "application/json".to_string()),
        ]);

        let (res_tx, res_rx) = channel(1);
        let (req_tx, req_rx) = channel(1);

        handle_requests(req_rx, res_tx);

        Self {
            input_mode: InputMode::Normal,
            endpoint: Input {
                text: String::from("https://fakestoreapi.com/products"),
                ..Input::default()
            },
            selected_header: 0,
            is_loading: false,
            headers,
            res_rx,
            req_tx,
            raw_body: Input::default(),
            method: RequestMethod::Get,
            request_tab: RequestTab::Body,
            selected_block: AppBlock::Endpoint,
            response: None,
            response_scroll: (0, 0),
            popup: None,
        }
    }
}
