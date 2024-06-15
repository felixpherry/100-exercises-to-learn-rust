pub mod field;

use field::status::HttpStatus;
use std::collections::HashMap;

const CLRF: &str = "\r\n";

pub struct Response {
    status: field::status::Status,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl Response {
    pub fn new(status: HttpStatus) -> Self {
        Self {
            status: status.into(),
            headers: HashMap::new(),
            body: None,
        }
    }

    pub fn http_string(&self) -> String {
        let status_line = self.status.status_line();
        let headers = self
            .headers
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<String>>()
            .join(CLRF);

        let body = self.body.clone().unwrap_or("".to_owned());

        [status_line, headers, body].join(CLRF)
    }
}
