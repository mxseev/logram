use std::fmt;
use std::error::Error;
use url;
use reqwest;
use serde_json;

use telegram::Response;


#[derive(Debug)]
pub enum TelegramError {
    Url(url::ParseError),
    Http(reqwest::Error),
    Json(serde_json::Error),
    Api(ApiError),
}
impl fmt::Display for TelegramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cause = match self.cause() {
            Some(e) => format!(" ({})", e),
            None => String::new(),
        };
        write!(f, "{}{}", self.description(), &cause)
    }
}
impl Error for TelegramError {
    fn description(&self) -> &str {
        match *self {
            TelegramError::Url(_) => "Could not make url",
            TelegramError::Http(_) => "Could not make http request",
            TelegramError::Json(_) => "Could not (de)serialize json",
            TelegramError::Api(_) => "Api error",
        }
    }
    fn cause(&self) -> Option<&Error> {
        match *self {
            TelegramError::Url(ref e) => Some(e),
            TelegramError::Http(ref e) => Some(e),
            TelegramError::Json(ref e) => Some(e),
            TelegramError::Api(ref e) => Some(e),
        }
    }
}
impl From<url::ParseError> for TelegramError {
    fn from(e: url::ParseError) -> Self {
        TelegramError::Url(e)
    }
}
impl From<reqwest::Error> for TelegramError {
    fn from(e: reqwest::Error) -> Self {
        TelegramError::Http(e)
    }
}
impl From<serde_json::Error> for TelegramError {
    fn from(e: serde_json::Error) -> Self {
        TelegramError::Json(e)
    }
}
impl From<ApiError> for TelegramError {
    fn from(e: ApiError) -> Self {
        TelegramError::Api(e)
    }
}

#[derive(Debug)]
pub struct ApiError {
    pub description: String,
}
impl<T> From<Response<T>> for ApiError {
    fn from(e: Response<T>) -> Self {
        let description = e.description.unwrap_or_else(
            || String::from("no description"),
        );
        ApiError { description }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}
impl Error for ApiError {
    fn description(&self) -> &str {
        self.description.as_str()
    }
    fn cause(&self) -> Option<&Error> {
        None
    }
}
