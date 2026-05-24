use crate::playlist::{PlaylistEntry, PlaylistError};
use url::Url;

pub fn parse_m3u(content: &str) -> Result<Vec<PlaylistEntry>, PlaylistError> {
    let content = content
        .trim_start()
        .strip_prefix('\u{feff}')
        .unwrap_or(content);
    let mut entries = Vec::new();
    let mut current_title: Option<String> = None;
    let mut found_extm3u = false;

    for line in content.lines() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        if !found_extm3u && line.starts_with('#') {
            if line.eq_ignore_ascii_case("#EXTM3U") {
                found_extm3u = true;
            }
            continue;
        }

        if let Some(rest) = line.strip_prefix("#EXTINF:") {
            if let Some(comma_pos) = rest.find(',') {
                current_title = Some(rest[comma_pos + 1..].trim().to_string());
            } else {
                current_title = None;
            }
            continue;
        }

        if line.starts_with('#') {
            current_title = None;
            continue;
        }

        if let Ok(url) = Url::parse(line) {
            entries.push(PlaylistEntry {
                url,
                title: current_title.take(),
            });
        }
    }

    if entries.is_empty() {
        return Err(PlaylistError::Parse(
            "No valid URLs found in M3U playlist".into(),
        ));
    }

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_extm3u_basic() {
        let m3u = "#EXTM3U\n#EXTINF:-1,Station Name\nhttp://example.com/stream\n";
        let entries = parse_m3u(m3u).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].url.as_str(), "http://example.com/stream");
        assert_eq!(entries[0].title.as_deref(), Some("Station Name"));
    }

    #[test]
    fn test_parse_extm3u_multiple() {
        let m3u = "#EXTM3U\n#EXTINF:-1,One\nhttp://one.com/stream\n#EXTINF:-1,Two\nhttp://two.com/stream\n";
        let entries = parse_m3u(m3u).unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].title.as_deref(), Some("One"));
        assert_eq!(entries[1].title.as_deref(), Some("Two"));
    }

    #[test]
    fn test_parse_bare_urls() {
        let m3u = "http://example.com/stream\nhttp://backup.com/stream\n";
        let entries = parse_m3u(m3u).unwrap();
        assert_eq!(entries.len(), 2);
        assert!(entries[0].title.is_none());
    }

    #[test]
    fn test_parse_mixed() {
        let m3u = "#EXTM3U\n#EXTINF:-1,Named\nhttp://named.com/stream\nhttp://bare.com/stream\n";
        let entries = parse_m3u(m3u).unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].title.as_deref(), Some("Named"));
        assert!(entries[1].title.is_none());
    }

    #[test]
    fn test_parse_utf8_bom() {
        let m3u = "\u{feff}#EXTM3U\n#EXTINF:-1,Test\nhttp://example.com/stream\n";
        let entries = parse_m3u(m3u).unwrap();
        assert_eq!(entries.len(), 1);
    }

    #[test]
    fn test_parse_blank_lines() {
        let m3u = "#EXTM3U\n\n#EXTINF:-1,Test\n\nhttp://example.com/stream\n\n";
        let entries = parse_m3u(m3u).unwrap();
        assert_eq!(entries.len(), 1);
    }

    #[test]
    fn test_parse_comment_only() {
        let m3u = "#EXTM3U\n# just a comment\n";
        let result = parse_m3u(m3u);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_empty() {
        let result = parse_m3u("");
        assert!(result.is_err());
    }
}
