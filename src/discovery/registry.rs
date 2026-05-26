use std::path::{Path, PathBuf};

use crate::discovery::provider::{DiscoveryProvider, parse_provider_metadata};

const SYSTEM_DIR: &str = "data/providers";
const USER_DIR: &str = ".local/share/shortwave/providers";

fn dirs_data_dir() -> Option<std::path::PathBuf> {
    std::env::var("XDG_DATA_HOME")
        .ok()
        .map(std::path::PathBuf::from)
        .or_else(|| {
            std::env::var("HOME")
                .ok()
                .map(|h| std::path::PathBuf::from(h).join(".local").join("share"))
        })
}

#[derive(Debug, Clone)]
pub struct ProviderRegistry {
    providers: Vec<DiscoveryProvider>,
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ProviderRegistry {
    pub fn new() -> Self {
        ProviderRegistry {
            providers: Vec::new(),
        }
    }

    pub fn scan_from(datadir: &Path, userdir: &Path) -> Self {
        let mut providers: Vec<DiscoveryProvider> = Vec::new();
        let mut seen_ids = std::collections::HashSet::new();

        let mut add_from = |dir: &Path, overwrite: bool| {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) != Some("rhai") {
                        continue;
                    }
                    let (name, description) = parse_provider_metadata(&path);
                    let id = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("unknown")
                        .to_string();

                    if overwrite && let Some(pos) = providers.iter().position(|p| p.id == id) {
                        providers[pos] = DiscoveryProvider {
                            id,
                            name,
                            description,
                            script_path: path,
                            enabled: true,
                        };
                        continue;
                    }

                    if seen_ids.insert(id.clone()) {
                        providers.push(DiscoveryProvider {
                            id,
                            name,
                            description,
                            script_path: path,
                            enabled: true,
                        });
                    }
                }
            }
        };

        add_from(datadir, false);
        add_from(userdir, true);

        ProviderRegistry { providers }
    }

    pub fn scan() -> Self {
        let datadir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(SYSTEM_DIR);
        let homedir = dirs_data_dir()
            .map(|d| d.join("shortwave").join("providers"))
            .unwrap_or_else(|| PathBuf::from(USER_DIR));

        Self::scan_from(&datadir, &homedir)
    }

    pub fn providers(&self) -> &[DiscoveryProvider] {
        &self.providers
    }

    pub fn enable(&mut self, id: &str) {
        if let Some(p) = self.providers.iter_mut().find(|p| p.id == id) {
            p.enabled = true;
        }
    }

    pub fn disable(&mut self, id: &str) {
        if let Some(p) = self.providers.iter_mut().find(|p| p.id == id) {
            p.enabled = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn create_temp_rhai(dir: &tempfile::TempDir, name: &str, header: &str) -> PathBuf {
        let path = dir.path().join(name);
        let mut f = std::fs::File::create(&path).unwrap();
        writeln!(f, "{}", header).unwrap();
        writeln!(f, "let x = 1;").unwrap();
        path
    }

    #[test]
    fn test_scan_finds_rhai_files() {
        let dir = tempfile::tempdir().unwrap();
        create_temp_rhai(&dir, "test.rhai", "// Discovery-Provider: Test");
        create_temp_rhai(&dir, "other.txt", "// not a script");

        let registry = ProviderRegistry::scan_from(dir.path(), Path::new("/nonexistent"));
        assert_eq!(registry.providers().len(), 1);
        assert_eq!(registry.providers()[0].name, "Test");
    }

    #[test]
    fn test_scan_ignores_non_rhai() {
        let dir = tempfile::tempdir().unwrap();
        create_temp_rhai(&dir, "script.rs", "// Discovery-Provider: Rust");
        create_temp_rhai(&dir, "data.txt", "");

        let registry = ProviderRegistry::scan_from(dir.path(), Path::new("/nonexistent"));
        assert_eq!(registry.providers().len(), 0);
    }

    #[test]
    fn test_user_overrides_bundled() {
        let sysdir = tempfile::tempdir().unwrap();
        let userdir = tempfile::tempdir().unwrap();
        create_temp_rhai(&sysdir, "same.rhai", "// Discovery-Provider: System");
        create_temp_rhai(&userdir, "same.rhai", "// Discovery-Provider: User");

        let registry = ProviderRegistry::scan_from(sysdir.path(), userdir.path());

        assert_eq!(registry.providers().len(), 1);
        assert_eq!(registry.providers()[0].name, "User");
    }

    #[test]
    fn test_provider_fallback_name() {
        let dir = tempfile::tempdir().unwrap();
        create_temp_rhai(&dir, "fallback_test.rhai", "// no header here");

        let registry = ProviderRegistry::scan_from(dir.path(), Path::new("/nonexistent"));
        assert_eq!(registry.providers().len(), 1);
        assert_eq!(registry.providers()[0].name, "fallback_test");
    }

    #[test]
    fn test_enable_disable() {
        let dir = tempfile::tempdir().unwrap();
        create_temp_rhai(&dir, "toggle.rhai", "// Discovery-Provider: Toggle");

        let mut registry = ProviderRegistry::scan_from(dir.path(), Path::new("/nonexistent"));
        assert!(registry.providers()[0].enabled);

        registry.disable("toggle");
        assert!(!registry.providers()[0].enabled);

        registry.enable("toggle");
        assert!(registry.providers()[0].enabled);
    }

    #[test]
    fn test_scan_missing_directory() {
        let registry = ProviderRegistry::scan_from(
            Path::new("/tmp/nonexistent_12345"),
            Path::new("/tmp/nonexistent_67890"),
        );
        assert_eq!(registry.providers().len(), 0);
    }
}
