use std::str::FromStr;

use reqwest::header::{HeaderMap, HeaderName};

use crate::app::{BodyContentType, Request, RequestMethod, Response};

pub async fn send(req: Request) -> Response {
    let method = match req.method {
        RequestMethod::Get => reqwest::Method::GET,
        RequestMethod::Post => reqwest::Method::POST,
        RequestMethod::Put => reqwest::Method::PUT,
        RequestMethod::Delete => reqwest::Method::DELETE,
        RequestMethod::Patch => reqwest::Method::PATCH,
    };

    let mut headers = HeaderMap::new();

    req.headers.iter().for_each(|(key, value)| {
        headers.insert(
            HeaderName::from_str(key.as_str()).unwrap(),
            value.parse().unwrap(),
        );
    });

    let client = reqwest::Client::new();

    let mut builder = client
        .request(method, &req.endpoint)
        .headers(headers)
        .query(&req.query_params);

    match req.body_content_type {
        BodyContentType::Text(_) => {
            if !req.body.trim().is_empty() {
                builder = builder.body(req.body);
            }
        }
        BodyContentType::Form => {
            builder = builder.form(&req.body_form);
        }
    }

    let response = builder.send().await.unwrap();

    let status_code = response.status().as_u16();

    let content_type = response.headers().get("content-type").unwrap();

    let text: String;

    let content_type_value: String;

    match content_type.clone().to_str().unwrap().to_lowercase() {
        h if h.contains("application/json") => {
            let data = response.json::<serde_json::Value>().await.unwrap();

            content_type_value = "application/json".to_string();
            text = format!("{:#}\n", data);
        }

        header_value => {
            content_type_value = header_value.clone().to_string();

            text = response.text().await.unwrap();
        }
    }

    Response {
        status_code,
        text,
        content_type: content_type_value,
    }
}
