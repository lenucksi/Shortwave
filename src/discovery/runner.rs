use rhai::Engine;

use crate::discovery::provider::DiscoveryProvider;
use crate::discovery::types::DiscoveryResult;

#[derive(Debug)]
pub enum RunnerError {
    Compile(String),
    Runtime(String),
    Parse(String),
    Timeout,
}

impl std::fmt::Display for RunnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunnerError::Compile(msg) => write!(f, "compile error: {msg}"),
            RunnerError::Runtime(msg) => write!(f, "runtime error: {msg}"),
            RunnerError::Parse(msg) => write!(f, "parse error: {msg}"),
            RunnerError::Timeout => write!(f, "script timed out"),
        }
    }
}

impl std::error::Error for RunnerError {}

pub fn run_provider(
    engine: &Engine,
    provider: &DiscoveryProvider,
) -> Result<DiscoveryResult, RunnerError> {
    let ast = engine
        .compile_file(provider.script_path.clone())
        .map_err(|e| RunnerError::Compile(e.to_string()))?;

    let json_str: String = engine
        .eval_ast(&ast)
        .map_err(|e| RunnerError::Runtime(e.to_string()))?;

    serde_json::from_str::<DiscoveryResult>(&json_str)
        .map_err(|e| RunnerError::Parse(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::discovery::engine::create;
    use std::io::Write;
    use std::path::PathBuf;

    fn write_script(dir: &tempfile::TempDir, name: &str, content: &str) -> PathBuf {
        let path = dir.path().join(name);
        let mut f = std::fs::File::create(&path).unwrap();
        writeln!(f, "{}", content).unwrap();
        path
    }

    #[test]
    fn test_run_valid_script() {
        let dir = tempfile::tempdir().unwrap();
        let path = write_script(
            &dir,
            "test.rhai",
            r#"
            json_stringify(#{
                "provider": "Test",
                "stations": [
                    #{
                        "name": "Test Station",
                        "stream_url": "https://example.com/stream",
                        "stream_urls": []
                    }
                ]
            })
        "#,
        );

        let provider = DiscoveryProvider {
            id: "test".into(),
            name: "Test".into(),
            description: String::new(),
            script_path: path,
            enabled: true,
        };

        let engine = create();
        let result = run_provider(&engine, &provider).unwrap();
        assert_eq!(result.provider, "Test");
        assert_eq!(result.stations.len(), 1);
        assert_eq!(result.stations[0].name, "Test Station");
    }

    #[test]
    fn test_run_invalid_script_returns_compile_error() {
        let dir = tempfile::tempdir().unwrap();
        let path = write_script(&dir, "bad.rhai", "this is not valid rhai @@@");

        let provider = DiscoveryProvider {
            id: "bad".into(),
            name: "Bad".into(),
            description: String::new(),
            script_path: path,
            enabled: true,
        };

        let engine = create();
        let result = run_provider(&engine, &provider);
        assert!(matches!(result, Err(RunnerError::Compile(_))));
    }

    #[test]
    fn test_run_script_with_runtime_error() {
        let dir = tempfile::tempdir().unwrap();
        let path = write_script(
            &dir,
            "crash.rhai",
            r#"
            let x = 1 / 0;
            json_stringify(x)
        "#,
        );

        let provider = DiscoveryProvider {
            id: "crash".into(),
            name: "Crash".into(),
            description: String::new(),
            script_path: path,
            enabled: true,
        };

        let engine = create();
        let result = run_provider(&engine, &provider);
        assert!(matches!(result, Err(RunnerError::Runtime(_))));
    }
}
