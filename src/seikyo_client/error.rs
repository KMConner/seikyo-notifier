use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct ApiError {
    pub(in crate::seikyo_client) error_msg: Option<String>,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.error_msg {
            None => write!(f, "Unknown error"),
            Some(msg) => write!(f, "API Error: {}", msg)
        }
    }
}

impl Error for ApiError {}

impl ApiError {
    pub fn new(msg: &str) -> ApiError {
        ApiError {
            error_msg: Some(String::from(msg))
        }
    }
}
