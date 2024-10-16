use minijinja::Value;
use reqwest::{Method, Url};
use reqwest::blocking::{Client, Response};
use serde_derive::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
struct Res {
    status: u16,
    headers: HashMap<String, String>,
    body: Vec<u8>
}

impl Res {
    fn new (response: Response) -> Value {
        let status = response.status().as_u16();

        let mut headers = HashMap::new();
        for key in response.headers().keys() {
            if let Some(value) = response.headers().get(key) {
                if let Ok(value) = value.to_str() {
                    headers.insert(key.to_string(), value.to_string());
                }
            }
        }

        match response.bytes() {
            Ok(body) => Value::from_serialize(Res {
                status,
                headers,
                body: body.to_vec()
            }),
            Err(_) => Value::from_serialize(Res {
                status,
                headers,
                body: Vec::new()
            })
        }
    }

    fn err (message: String) -> Value {
        Value::from_serialize(Res {
            status: 400,
            headers: HashMap::new(),
            body: message.as_bytes().to_vec()
        })
    }
}

fn fetch (
    method: &str,
    url: &str,
    body: Option<&Vec<u8>> 
) -> Value {
    let method: Method = match method.parse() {
        Ok(method) => method,
        Err(err) => {
            return Res::err(format!("Invalid method!\n{:#?}", err));
        }
    };

    let url: Url = match url.parse() {
        Ok(url) => url,
        Err(err) => {
            return Res::err(format!("Invalid URL!\n{:#?}", err));
        }
    };

    let mut request = Client::new().request(method, url);
    if let Some(body) = body {
        request = request.body(body.to_vec());
    }

    match request.send() {
        Ok(response) => Res::new(response),
        Err(err) => Res::err(format!("Request fail!\n{:#?}", err))
    }
}

pub fn get (url: &str) -> Value {
    fetch("GET", url, None)
}

pub fn delete (url: &str) -> Value {
    fetch("DELETE", url, None)
}

pub fn head (url: &str) -> Value {
    fetch("HEAD", url, None)
}

pub fn options (url: &str) -> Value {
    fetch("OPTIONS", url, None)
}

pub fn post (url: &str, body: &Vec<u8>) -> Value {
    fetch("POST", url, Some(body))
}

pub fn put (url: &str, body: &Vec<u8>) -> Value {
    fetch("PUT", url, Some(body))
}

pub fn patch (url: &str, body: &Vec<u8>) -> Value {
    fetch("PATCH", url, Some(body))
}
