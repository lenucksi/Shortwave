#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreamUrlInfo {
    pub url: String,
    pub codec: Option<String>,
    pub bitrate: Option<i32>,
    pub tls: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StationData {
    pub name: String,
    pub stream_url: String,
    #[serde(default)]
    pub stream_urls: Vec<StreamUrlInfo>,
    pub homepage: Option<String>,
    pub icon_url: Option<String>,
    pub tags: Option<String>,
    pub country: Option<String>,
    pub language: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiscoveryResult {
    pub provider: String,
    pub stations: Vec<StationData>,
}
