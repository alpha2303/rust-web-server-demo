pub struct HttpResponse {
    pub status: String,
    pub reason: String,
    pub headers: String,
    pub body: String,
}

impl HttpResponse {
    pub fn new(status: &str, headers: &str, body: &str) -> Self {
        HttpResponse {
            status: status.to_string(),
            reason: HttpResponse::get_status_line(status.to_string()).to_string(),
            headers: headers.to_string(),
            body: body.to_string(),
        }
    }

    fn get_status_line(status: String) -> String {
        let reason_phrase: &str = match status.as_str() {
            "200" => "OK",
            "404" => "NOT FOUND",
            "400" => "BAD REQUEST",
            _ => "UNKNOWN",
        };
        reason_phrase.to_string()
    }

    pub fn response_body(&self) -> String {
        format!(
            "HTTP/1.1 {} {}\r\nContent-Length {}\r\n\r\n{}",
            self.status,
            self.reason,
            self.body.len(),
            self.body
        )
    }
}
