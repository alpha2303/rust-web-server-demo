use std::{collections::HashMap, hash::Hash};

use crate::rusty_server::helpers::methods::unparse_headers;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Unsupported,
}

impl HttpMethod {
    pub fn get_method(method: &str) -> Self {
        match method {
            "GET" => HttpMethod::Get,
            "HEAD" => HttpMethod::Head,
            "POST" => HttpMethod::Post,
            "PUT" => HttpMethod::Put,
            "DELETE" => HttpMethod::Delete,
            _ => HttpMethod::Unsupported,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Head => "HEAD",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
            _ => "Unsupported",
        }
    }
}

#[derive(Debug)]
pub struct HttpRequest {
    method: HttpMethod,
    uri: String,
    version: String,
    headers: HashMap<String, String>,
    body: String,
}

impl HttpRequest {
    pub fn from_request(request: &str) -> Self {
        let request_lines: Vec<&str> = request.split("\r\n").collect();
        let (method, uri, version) = HttpRequest::parse_metadata(request_lines[0]);
        let (headers, start_index) = HttpRequest::parse_headers(&request_lines);
        HttpRequest {
            method,
            uri,
            version,
            headers,
            body: HttpRequest::parse_body(request_lines, start_index),
        }
    }

    pub fn get_uri(&self) -> &String {
        &self.uri
    }

    pub fn get_method(&self) -> &HttpMethod {
        &self.method
    }

    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    fn parse_metadata(request_uri: &str) -> (HttpMethod, String, String) {
        let metadata: Vec<&str> = request_uri.split(" ").collect();
        let method: HttpMethod = HttpMethod::get_method(metadata[0]);
        let uri = metadata[1].to_string();
        let version = metadata[2].to_string();
        (method, uri, version)
    }

    fn parse_headers(request_lines: &Vec<&str>) -> (HashMap<String, String>, usize) {
        let mut headers: HashMap<String, String> = HashMap::new();
        let mut start_index = 1;
        while !(request_lines[start_index] == "") {
            match request_lines[start_index].split_once(": ") {
                Some((key, value)) => {
                    let _output = headers.insert(key.to_string(), value.to_string());
                }
                None => {}
            }
            start_index = start_index + 1;
        }
        (headers, start_index)
    }

    fn parse_body(request_lines: Vec<&str>, start_index: usize) -> String {
        request_lines[(start_index + 1)..]
            .join("\r\n")
            .trim_matches('\0')
            .to_string()
    }
}

pub struct HttpResponse {
    pub status: usize,
    pub reason: &'static str,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpResponse {
    pub fn new(status: usize, headers: Option<HashMap<String, String>>, body: &str) -> Self {
        HttpResponse {
            status: status,
            reason: HttpResponse::get_status_line(status),
            headers: match headers {
                Some(headers) => headers,
                None => HashMap::new(),
            },
            body: body.to_string(),
        }
    }

    fn get_status_line(status: usize) -> &'static str {
        let reason_phrase: &'static str = match status {
            200 => "OK",
            404 => "NOT FOUND",
            400 => "BAD REQUEST",
            _ => "UNKNOWN",
        };
        reason_phrase
    }

    pub fn response_body(&self) -> String {
        format!(
            "HTTP/1.1 {} {}\r\n{}\r\n\r\n{}",
            self.status,
            self.reason,
            unparse_headers(&self.headers),
            self.body
        )
    }
}
