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
            _ => Err(RequestError::InvalidHttpMethod),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Request {
    method: RequestMethod,
    headers: HashMap<String, String>,
    endpoint: String,
    body: Option<String>,
}

impl Request {
    pub fn new(
        method: RequestMethod,
        headers: HashMap<String, String>,
        endpoint: String,
        body: Option<String>,
    ) -> Self {
        Self {
            method,
            headers,
            endpoint,
            body,
        }
    }

    pub fn method(&self) -> &RequestMethod {
        &self.method
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn body(&self) -> &Option<String> {
        &self.body
    }

    pub fn get_header(&self, key: &str) -> Option<&String> {
        self.headers.get(key)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RequestError {
    #[error("Failed to parse HTTP string")]
    ParseError,
    #[error("Invalid HTTP method")]
    InvalidHttpMethod,
    #[error("Invalid endpoint")]
    InvalidEndpoint,
    #[error("Invalid headers")]
    InvalidHeaders,
}

const CLRF: &str = "\r\n";

impl TryFrom<&str> for Request {
    type Error = RequestError;
    fn try_from(request_string: &str) -> Result<Self, Self::Error> {
        let request_string = request_string.replace("\r\n\r\n", CLRF);
        let mut lines = request_string.split(CLRF);
        let request_line = lines.next().ok_or(RequestError::ParseError)?;
        let body = lines
            .clone()
            .last()
            .map(|l| l.to_owned())
            .filter(|l| !l.is_empty());

        let mut headers = HashMap::<String, String>::new();
        for line in lines.clone().skip(1) {
            if let Some((k, v)) = line.split_once(": ") {
                headers.insert(k.to_owned(), v.to_owned());
            } else {
                break;
            }
        }

        let mut request_line_iter = request_line.splitn(3, " ");
        let method: RequestMethod = request_line_iter
            .next()
            .ok_or(RequestError::InvalidHttpMethod)?
            .try_into()?;

        let endpoint = request_line_iter
            .next()
            .ok_or(RequestError::InvalidEndpoint)?
            .to_owned();

        Ok(Self::new(method, headers, endpoint, body))
    }
}
