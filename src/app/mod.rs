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

#[derive(Clone, PartialEq)]
pub enum RequestMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl ToString for RequestMethod {
    fn to_string(&self) -> String {
        match self {
            Self::Get => "GET".to_string(),
            Self::Post => "POST".to_string(),
            Self::Put => "PUT".to_string(),
            Self::Delete => "DELETE".to_string(),
            Self::Patch => "PATCH".to_string(),
        }
    }
}

pub trait OrderNavigation: Clone + PartialEq {
    fn get_order(&self) -> Vec<Self>
    where
        Self: Sized;
    fn next(&self) -> Self
    where
        Self: Sized,
    {
        let order = self.get_order();

        return order[(order.iter().position(|r| r == self).unwrap() + 1) % order.len()].clone();
    }
    fn previous(&self) -> Self
    where
        Self: Sized,
    {
        let order = self.get_order();

        let index = order.iter().position(|r| r == self).unwrap();

        if index == 0 {
            return order[order.len() - 1].clone();
        }

        return order[index - 1].clone();
    }

    fn get_index(&self) -> usize
    where
        Self: Sized,
    {
        self.get_order().iter().position(|r| r == self).unwrap()
    }
}

impl OrderNavigation for RequestMethod {
    fn get_order(&self) -> Vec<Self> {
        vec![Self::Get, Self::Post, Self::Put, Self::Patch, Self::Delete]
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

#[derive(Clone, PartialEq)]
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

impl OrderNavigation for RequestTab {
    fn get_order(&self) -> Vec<Self> {
        vec![
            Self::Body,
            Self::Query,
            Self::Headers,
            Self::Auth,
            Self::Cookies,
        ]
    }
}

impl OrderNavigation for AppBlock {
    fn get_order(&self) -> Vec<Self> {
        vec![
            Self::Method,
            Self::Endpoint,
            Self::Request,
            Self::RequestContent,
            Self::Response,
        ]
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
