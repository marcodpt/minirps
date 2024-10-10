use minijinja::{Error, ErrorKind::InvalidOperation, Value};
use reqwest::{Method, Url, header};
use reqwest::blocking::Client;

fn fetch (
    method: &str,
    url: &str,
    params: Option<&Value> 
) -> Result<Value, Error> {
    let info = format!("{} {} {:#?}", method, url, params);

    let method: Method = match method.parse() {
        Ok(method) => method,
        Err(err) => {
            return Err(Error::new(
                InvalidOperation,
                format!("Invalid method!\n{}\n{:#}", info, err)
            ));
        }
    };

    let url: Url = match url.parse() {
        Ok(url) => url,
        Err(err) => {
            return Err(Error::new(
                InvalidOperation,
                format!("Invalid URL!\n{}\n{:#}", info, err)
            ));
        }
    };

    let request = Client::new().request(method, url);

    match request.send() {
        Ok(response) => {
            if response.status().is_success() {
                let content = response.headers().get(header::CONTENT_TYPE);

                let is_json = if let Some(content) = content {
                    let v = b"application/json;";
                    let l = v.len();
                    let c = content.as_bytes();
                    *c == v[0..(l - 1)] || c[0..l] == *v
                } else {false};

                if is_json {
                    match response.json::<Value>() {
                        Ok(json) => Ok(json),
                        Err(err) => Err(Error::new(
                            InvalidOperation,
                            format!(
                                "Fail to parse response json!\n{}\n{:#}",
                                info, err
                            )
                        ))
                    }
                } else {
                    match response.text() {
                        Ok(text) => Ok(text.into()),
                        Err(err) => Err(Error::new(
                            InvalidOperation,
                            format!(
                                "Fail to parse response text!\n{}\n{:#}",
                                info, err
                            )
                        ))
                    }
                }
            } else {
                Ok(response.status().as_u16().into())
            }
        },
        Err(err) => Err(Error::new(
            InvalidOperation,
            format!("Request fail!\n{}\n{:#}", info, err)
        ))
    }
}

pub fn get (url: &str) -> Result<Value, Error> {
    fetch("GET", url, None)
}

pub fn delete (url: &str) -> Result<Value, Error> {
    fetch("DELETE", url, None)
}

pub fn head (url: &str) -> Result<Value, Error> {
    fetch("HEAD", url, None)
}

pub fn options (url: &str) -> Result<Value, Error> {
    fetch("OPTIONS", url, None)
}

pub fn post (url: &str, params: &Value) -> Result<Value, Error> {
    fetch("POST", url, Some(params))
}

pub fn put (url: &str, params: &Value) -> Result<Value, Error> {
    fetch("PUT", url, Some(params))
}

pub fn patch (url: &str, params: &Value) -> Result<Value, Error> {
    fetch("PATCH", url, Some(params))
}
