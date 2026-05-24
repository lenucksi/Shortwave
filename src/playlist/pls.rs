use crate::playlist::{PlaylistEntry, PlaylistError};
use std::collections::HashMap;
use url::Url;

pub fn parse_pls(content: &str) -> Result<Vec<PlaylistEntry>, PlaylistError> {
    let mut entries: HashMap<usize, (Option<String>, Option<String>)> = HashMap::new();
    let mut in_playlist = false;
    let mut num_entries: Option<usize> = None;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.eq_ignore_ascii_case("[playlist]") {
            in_playlist = true;
            continue;
        }

        if !in_playlist {
            continue;
        }

        if let Some(eq_pos) = line.find('=') {
            let key = line[..eq_pos].trim();
            let value = line[eq_pos + 1..].trim();

            if key.starts_with("NumberOfEntries") {
                num_entries = value.parse().ok();
                continue;
            }

            if let Some(stripped) = key.strip_prefix("File") {
                let index: usize = stripped.parse().unwrap_or(1);
                if Url::parse(value).is_ok() {
                    entries
                        .entry(index)
                        .and_modify(|e| e.0 = Some(value.to_string()))
                        .or_insert((Some(value.to_string()), None));
                }
            } else if let Some(stripped) = key.strip_prefix("Title") {
                let index: usize = stripped.parse().unwrap_or(1);
                entries
                    .entry(index)
                    .and_modify(|e| e.1 = Some(value.to_string()))
                    .or_insert((None, Some(value.to_string())));
            }
        }
    }

    if entries.is_empty() {
        return Err(PlaylistError::Parse(
            "No playlist entries found in PLS".into(),
        ));
    }

    let expected_count = num_entries.unwrap_or(entries.len());
    let mut result = Vec::with_capacity(expected_count);

    for i in 1..=expected_count {
        if let Some((Some(url_str), title)) = entries.remove(&i)
            && let Ok(url) = Url::parse(&url_str)
        {
            result.push(PlaylistEntry { url, title });
        }
    }

    if result.is_empty() {
        return Err(PlaylistError::Parse("No valid URLs found in PLS".into()));
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_pls() {
        let pls = "[playlist]\nFile1=http://example.com/stream\nTitle1=Test\nNumberOfEntries=1\nVersion=2";
        let entries = parse_pls(pls).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].url.as_str(), "http://example.com/stream");
        assert_eq!(entries[0].title.as_deref(), Some("Test"));
    }

    #[test]
    fn test_parse_multiple_entries() {
        let pls = "[playlist]\nFile1=http://primary.com/stream\nFile2=http://backup.com/stream\nTitle1=Station\nNumberOfEntries=2";
        let entries = parse_pls(pls).unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].url.as_str(), "http://primary.com/stream");
        assert_eq!(entries[1].url.as_str(), "http://backup.com/stream");
    }

    #[test]
    fn test_parse_with_comments() {
        let pls =
            "# this is a comment\n[playlist]\nFile1=http://example.com/stream\nNumberOfEntries=1";
        let entries = parse_pls(pls).unwrap();
        assert_eq!(entries.len(), 1);
    }

    #[test]
    fn test_parse_windows_crlf() {
        let pls = "[playlist]\r\nFile1=http://example.com/stream\r\nNumberOfEntries=1\r\n";
        let entries = parse_pls(pls).unwrap();
        assert_eq!(entries.len(), 1);
    }

    #[test]
    fn test_parse_missing_number_of_entries() {
        let pls = "[playlist]\nFile1=http://example.com/stream\nFile2=http://backup.com/stream";
        let entries = parse_pls(pls).unwrap();
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn test_parse_extra_whitespace() {
        let pls =
            "[playlist]\nFile1 = http://example.com/stream\nTitle1 = Station\nNumberOfEntries=1";
        let entries = parse_pls(pls).unwrap();
        assert_eq!(entries.len(), 1);
    }

    #[test]
    fn test_parse_empty_content() {
        let result = parse_pls("");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_gibberish() {
        let result = parse_pls("aklsjdflkajsdf lkajsdf lkajsdf");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_no_file_entries() {
        let pls = "[playlist]\nTitle1=Test\nNumberOfEntries=1";
        let result = parse_pls(pls);
        assert!(result.is_err());
    }
}
