use crate::playlist::{PlaylistEntry, PlaylistError};
use url::Url;

pub async fn fetch_and_parse(url: &Url) -> Result<Vec<PlaylistEntry>, PlaylistError> {
    let response = crate::api::http::get(url.clone()).await?;
    let status = response.status();
    let content = response.text().await.map_err(PlaylistError::Http)?;

    if !status.is_success() {
        return Err(PlaylistError::Parse(format!("Server returned {status}")));
    }

    let path = url.path().to_lowercase();
    if path.ends_with(".pls") || content.trim().starts_with("[playlist]") {
        crate::playlist::parse_pls(&content)
    } else {
        crate::playlist::parse_m3u(&content)
    }
}
