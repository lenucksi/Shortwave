mod fetch;
mod m3u;
mod pls;

use std::fmt;
use url::Url;

#[derive(Debug, Clone)]
pub struct PlaylistEntry {
    pub url: Url,
    pub title: Option<String>,
}

#[derive(Debug)]
pub enum PlaylistError {
    Http(reqwest::Error),
    Parse(String),
    #[allow(dead_code)]
    UnsupportedFormat,
}

impl fmt::Display for PlaylistError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http(e) => write!(f, "HTTP error: {e}"),
            Self::Parse(msg) => write!(f, "Parse error: {msg}"),
            Self::UnsupportedFormat => write!(f, "Unsupported playlist format"),
        }
    }
}

impl std::error::Error for PlaylistError {}

impl From<reqwest::Error> for PlaylistError {
    fn from(e: reqwest::Error) -> Self {
        Self::Http(e)
    }
}

pub use fetch::fetch_and_parse;
pub use m3u::parse_m3u;
pub use pls::parse_pls;
