#[derive(Debug, thiserror::Error)]
pub enum StatusError {
    #[error("status code is invalid")]
    InvalidStatusCode,
}

#[derive(Debug, Clone)]
pub enum HttpStatus {
    Ok,
    NotFound,
    Created,
}

#[derive(Debug, Clone)]
pub struct Status(HttpStatus);

impl Status {
    fn new(http_status: HttpStatus) -> Self {
        Self(http_status)
    }

    pub fn status_code(&self) -> u16 {
        match self.0 {
            HttpStatus::Ok => 200,
            HttpStatus::NotFound => 404,
            HttpStatus::Created => 201,
        }
    }

    pub fn status_line(&self) -> String {
        match self.0 {
            HttpStatus::Ok => "HTTP/1.1 200 OK".to_owned(),
            HttpStatus::NotFound => "HTTP/1.1 404 Not Found".to_owned(),
            HttpStatus::Created => "HTTP/1.1 201 Created".to_owned(),
        }
    }
}

impl From<HttpStatus> for Status {
    fn from(status: HttpStatus) -> Self {
        Self::new(status)
    }
}

impl TryFrom<u16> for Status {
    type Error = StatusError;
    fn try_from(status: u16) -> Result<Self, Self::Error> {
        let http_status: HttpStatus = status.try_into()?;
        Ok(Status::new(http_status))
    }
}

impl TryFrom<u16> for HttpStatus {
    type Error = StatusError;
    fn try_from(status: u16) -> Result<Self, Self::Error> {
        match status {
            200 => Ok(Self::Ok),
            _ => Err(Self::Error::InvalidStatusCode),
        }
    }
}
