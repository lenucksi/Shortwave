use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct DiscoveryProvider {
    pub id: String,
    pub name: String,
    pub description: String,
    pub script_path: PathBuf,
    pub enabled: bool,
}

pub fn parse_provider_metadata(path: &Path) -> (String, String) {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => {
            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();
            return (name, String::new());
        }
    };

    let name = content
        .lines()
        .find_map(|l| l.strip_prefix("// Discovery-Provider:"))
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| {
            path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string()
        });

    let description = content
        .lines()
        .find_map(|l| l.strip_prefix("// Description:"))
        .map(|s| s.trim().to_string())
        .unwrap_or_default();

    (name, description)
}
