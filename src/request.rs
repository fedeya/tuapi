use std::str::FromStr;

use reqwest::header::{HeaderMap, HeaderName};

use crate::app::{App, RequestMethod, Response};

pub fn handle_request(app: &mut App) {
    let method = match app.method {
        RequestMethod::Get => reqwest::Method::GET,
        RequestMethod::Post => reqwest::Method::POST,
        RequestMethod::Put => reqwest::Method::PUT,
        RequestMethod::Delete => reqwest::Method::DELETE,
    };

    let mut headers = HeaderMap::new();

    app.headers.iter().for_each(|(key, value)| {
        headers.insert(
            HeaderName::from_str(key.as_str()).unwrap(),
            value.parse().unwrap(),
        );
    });

    let client = reqwest::blocking::Client::new();

    let builder = client
        .request(method, &app.endpoint.text)
        .body(app.raw_body.text.clone())
        .headers(headers);

    let response = builder.send().unwrap();

    let status_code = response.status().as_u16();

    let content_type = response.headers().get("content-type").unwrap();

    let text: String;

    match content_type.to_str().unwrap() {
        "application/json" => {
            let data = response.json::<serde_json::Value>().unwrap();

            text = format!("{:#}\n", data);
        }

        _ => {
            text = response.text().unwrap();
        }
    }

    app.response = Some(Response { status_code, text });
}
