use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum RequestMethod {
    Get,
    Post,
}

impl TryFrom<&str> for RequestMethod {
    type Error = RequestError;
    fn try_from(method: &str) -> Result<Self, Self::Error> {
        match method.to_lowercase().as_str() {
            "get" => Ok(Self::Get),
            "post" => Ok(Self::Post),
            _ => Err(Self::Error::ParseError),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Request {
    method: RequestMethod,
    endpoint: String,
    http_version: String,
    headers: HashMap<String, String>,
    body: String,
}

#[derive(Debug, thiserror::Error)]
pub enum RequestError {
    #[error("Failed to parse HTTP request")]
    ParseError,
}

impl Request {
    pub fn new(http_string: &str) -> Result<Self, RequestError> {
        let http_string = http_string.replace("\r\n\r\n", "\r\n");

        let request_vec = http_string.split("\r\n").collect::<Vec<_>>();

        let request_line = *request_vec.first().ok_or(RequestError::ParseError)?;
        let request_body = *request_vec.last().ok_or(RequestError::ParseError)?;

        let request_line_vec = request_line.split(" ").collect::<Vec<_>>();
        let http_method: RequestMethod =
            (*request_line_vec.get(0).ok_or(RequestError::ParseError)?).try_into()?;

        let endpoint = *request_line_vec.get(1).ok_or(RequestError::ParseError)?;
        let http_version = *request_line_vec.get(2).ok_or(RequestError::ParseError)?;
        Ok(Self {
            method: http_method,
            body: request_body.to_owned(),
            endpoint: endpoint.into(),
            http_version: http_version.into(),
            headers: HashMap::new(),
        })
    }

    pub fn method(&self) -> RequestMethod {
        self.method.clone()
    }

    pub fn endpoint(&self) -> String {
        self.endpoint.clone()
    }
}

impl TryFrom<&str> for Request {
    type Error = RequestError;
    fn try_from(http_string: &str) -> Result<Self, Self::Error> {
        Self::new(http_string)
    }
}
