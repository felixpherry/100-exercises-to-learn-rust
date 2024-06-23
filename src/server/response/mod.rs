pub mod field;

use field::status::HttpStatus;
use std::collections::HashMap;

const CLRF: &str = "\r\n";

#[derive(Debug, Clone)]
pub struct Response {
    status: field::status::Status,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl Response {
    pub fn new(status: HttpStatus, body: Option<String>) -> Self {
        Self {
            status: status.into(),
            headers: HashMap::new(),
            body,
        }
    }

    pub fn set_headers(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

    pub fn set_body(&mut self, body: Option<String>) {
        self.body = body;
    }

    pub fn http_string(&self) -> String {
        let status_line = self.status.status_line();
        let headers = self
            .headers
            .iter()
            .map(|(k, v)| format!("{}: {}\r\n", k, v))
            .collect::<Vec<String>>()
            .join("");

        let body = self.body.clone().unwrap_or("".to_owned());

        [status_line, headers, body].join(CLRF)
    }
}
