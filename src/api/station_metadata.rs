// Shortwave - station_metadata.rs
// Copyright (C) 2021-2022  Felix Häcker <haeckerfelix@gnome.org>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::fmt::Write;
use std::str::FromStr;

use cruet::Inflector;
use gtk::glib;
use serde::{Deserialize, Deserializer, Serializer};
use url::Url;

#[derive(glib::Boxed, Default, Debug, Clone, Serialize, Deserialize)]
#[boxed_type(name = "SwStationMetadata")]
pub struct StationMetadata {
    pub changeuuid: String,
    pub stationuuid: String,
    pub name: String,
    #[serde(serialize_with = "url_to_str")]
    #[serde(deserialize_with = "str_to_url")]
    pub url: Option<Url>,
    #[serde(serialize_with = "url_to_str")]
    #[serde(deserialize_with = "str_to_url")]
    pub url_resolved: Option<Url>,
    #[serde(
        default,
        serialize_with = "urls_to_vec",
        deserialize_with = "vec_to_urls"
    )]
    pub alternate_urls: Vec<Url>,
    #[serde(
        default,
        serialize_with = "url_to_str",
        deserialize_with = "str_to_url"
    )]
    pub playlist_url: Option<Url>,
    #[serde(default)]
    pub fetched_at: Option<i64>,
    #[serde(serialize_with = "url_to_str")]
    #[serde(deserialize_with = "str_to_url")]
    pub homepage: Option<Url>,
    #[serde(serialize_with = "url_to_str")]
    #[serde(deserialize_with = "str_to_url")]
    pub favicon: Option<Url>,
    pub tags: String,
    pub country: String,
    pub countrycode: String,
    pub state: String,
    pub language: String,
    pub languagecodes: String,
    pub votes: i32,
    pub lastchangetime: Option<String>,
    pub lastchangetime_iso8601: Option<String>,
    pub codec: String,
    pub bitrate: i32,
    pub hls: i32,
    pub lastcheckok: i32,
    pub lastchecktime: Option<String>,
    pub lastchecktime_iso8601: Option<String>,
    pub lastcheckoktime: Option<String>,
    pub lastcheckoktime_iso8601: Option<String>,
    pub lastlocalchecktime: Option<String>,
    pub clicktimestamp: Option<String>,
    pub clicktimestamp_iso8601: Option<String>,
    pub clickcount: i32,
    pub clicktrend: i32,
    pub ssl_error: i32,
    pub geo_lat: Option<f32>,
    pub geo_long: Option<f32>,
    pub has_extended_info: bool,
}

impl StationMetadata {
    pub fn formatted_tags(&self) -> String {
        let tags = self.tags.split(',');
        let mut formatted = String::new();
        for tag in tags {
            write!(formatted, ", {}", tag.to_title_case()).unwrap_or_default();
        }
        formatted.split_at(2).1.to_string()
    }
}

impl StationMetadata {
    /// Create metadata for a new local station.
    pub fn new(name: String, url: Url) -> Self {
        Self {
            name,
            url: Some(url),
            ..Default::default()
        }
    }
}

fn url_to_str<S>(url: &Option<Url>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let value = if let Some(url) = url {
        url.as_str()
    } else {
        ""
    };
    serializer.serialize_str(value)
}

fn str_to_url<'de, D>(deserializer: D) -> Result<Option<Url>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(Url::from_str(&s).ok())
}

fn urls_to_vec<S>(urls: &[Url], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use serde::ser::SerializeSeq;
    let mut seq = serializer.serialize_seq(Some(urls.len()))?;
    for url in urls {
        seq.serialize_element(url.as_str())?;
    }
    seq.end()
}

fn vec_to_urls<'de, D>(deserializer: D) -> Result<Vec<Url>, D::Error>
where
    D: Deserializer<'de>,
{
    let strings = Vec::<String>::deserialize(deserializer)?;
    Ok(strings
        .iter()
        .filter_map(|s| Url::from_str(s).ok())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_serialize_deserialize_default() {
        let metadata = StationMetadata::default();
        let json = serde_json::to_string(&metadata).expect("serialize");
        let deserialized: StationMetadata = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(metadata.name, deserialized.name);
        assert_eq!(metadata.homepage, deserialized.homepage);
        assert_eq!(metadata.tags, deserialized.tags);
        assert_eq!(metadata.country, deserialized.country);
        assert_eq!(metadata.language, deserialized.language);
        assert_eq!(metadata.codec, deserialized.codec);
        assert_eq!(metadata.state, deserialized.state);
        assert_eq!(metadata.alternate_urls, deserialized.alternate_urls);
        assert_eq!(metadata.playlist_url, deserialized.playlist_url);
        assert_eq!(metadata.fetched_at, deserialized.fetched_at);
    }

    #[test]
    fn test_serialize_deserialize_with_urls() {
        let metadata = StationMetadata {
            alternate_urls: vec![
                Url::from_str("https://example.com/stream1").unwrap(),
                Url::from_str("https://example.com/stream2").unwrap(),
            ],
            playlist_url: Some(Url::from_str("https://example.com/playlist.m3u8").unwrap()),
            fetched_at: Some(1234567890),
            ..Default::default()
        };
        let json = serde_json::to_string(&metadata).expect("serialize");
        let deserialized: StationMetadata = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(metadata.alternate_urls, deserialized.alternate_urls);
        assert_eq!(metadata.playlist_url, deserialized.playlist_url);
        assert_eq!(metadata.fetched_at, deserialized.fetched_at);
    }

    #[test]
    fn test_backward_compat_no_alternate_urls() {
        let json = serde_json::json!({
            "changeuuid": "",
            "stationuuid": "",
            "name": "test",
            "url": "",
            "url_resolved": "",
            "homepage": "",
            "favicon": "",
            "tags": "",
            "country": "",
            "countrycode": "",
            "state": "",
            "language": "",
            "languagecodes": "",
            "votes": 0,
            "lastchangetime": null,
            "lastchangetime_iso8601": null,
            "codec": "",
            "bitrate": 0,
            "hls": 0,
            "lastcheckok": 0,
            "lastchecktime": null,
            "lastchecktime_iso8601": null,
            "lastcheckoktime": null,
            "lastcheckoktime_iso8601": null,
            "lastlocalchecktime": null,
            "clicktimestamp": null,
            "clicktimestamp_iso8601": null,
            "clickcount": 0,
            "clicktrend": 0,
            "ssl_error": 0,
            "geo_lat": null,
            "geo_long": null,
            "has_extended_info": false
        });
        let metadata: StationMetadata = serde_json::from_value(json).expect("deserialize");
        assert!(metadata.alternate_urls.is_empty());
        assert!(metadata.playlist_url.is_none());
        assert!(metadata.fetched_at.is_none());
    }

    #[test]
    fn test_backward_compat_partial() {
        let json = serde_json::json!({
            "changeuuid": "",
            "stationuuid": "",
            "name": "test",
            "url": "",
            "url_resolved": "",
            "homepage": "",
            "favicon": "",
            "tags": "",
            "country": "",
            "countrycode": "",
            "state": "",
            "language": "",
            "languagecodes": "",
            "votes": 0,
            "lastchangetime": null,
            "lastchangetime_iso8601": null,
            "codec": "",
            "bitrate": 0,
            "hls": 0,
            "lastcheckok": 0,
            "lastchecktime": null,
            "lastchecktime_iso8601": null,
            "lastcheckoktime": null,
            "lastcheckoktime_iso8601": null,
            "lastlocalchecktime": null,
            "clicktimestamp": null,
            "clicktimestamp_iso8601": null,
            "clickcount": 0,
            "clicktrend": 0,
            "ssl_error": 0,
            "geo_lat": null,
            "geo_long": null,
            "has_extended_info": false,
            "alternate_urls": ["https://example.com/alt1"]
        });
        let metadata: StationMetadata = serde_json::from_value(json).expect("deserialize");
        assert_eq!(metadata.alternate_urls.len(), 1);
        assert!(metadata.playlist_url.is_none());
        assert!(metadata.fetched_at.is_none());
    }
}
