#![allow(dead_code)]

use rhai::{Dynamic, Engine, EvalAltResult};
use serde_json::Value;

fn http_get_with_client(
    client: &reqwest::blocking::Client,
    url: &str,
) -> Result<String, Box<EvalAltResult>> {
    let resp = client.get(url).send().map_err(|e| {
        Box::new(EvalAltResult::ErrorRuntime(
            format!("http_get: {e}").into(),
            rhai::Position::NONE,
        ))
    })?;
    resp.text().map_err(|e| {
        Box::new(EvalAltResult::ErrorRuntime(
            format!("http_get body: {e}").into(),
            rhai::Position::NONE,
        ))
    })
}

fn http_get_json_with_client(
    client: &reqwest::blocking::Client,
    url: &str,
) -> Result<Dynamic, Box<EvalAltResult>> {
    let body = http_get_with_client(client, url)?;
    let val: Value = serde_json::from_str(&body).map_err(|e| {
        Box::new(EvalAltResult::ErrorRuntime(
            format!("http_get_json parse: {e}").into(),
            rhai::Position::NONE,
        ))
    })?;
    rhai::serde::to_dynamic(&val).map_err(|e| {
        Box::new(EvalAltResult::ErrorRuntime(
            format!("http_get_json to_dynamic: {e}").into(),
            rhai::Position::NONE,
        ))
    })
}

fn json_parse(s: &str) -> Result<Dynamic, Box<EvalAltResult>> {
    let val: Value = serde_json::from_str(s).map_err(|e| {
        Box::new(EvalAltResult::ErrorRuntime(
            format!("json_parse: {e}").into(),
            rhai::Position::NONE,
        ))
    })?;
    rhai::serde::to_dynamic(&val).map_err(|e| {
        Box::new(EvalAltResult::ErrorRuntime(
            format!("json_parse to_dynamic: {e}").into(),
            rhai::Position::NONE,
        ))
    })
}

fn json_stringify(val: Dynamic) -> Result<String, Box<EvalAltResult>> {
    let val: Value = rhai::serde::from_dynamic(&val).map_err(|e| {
        Box::new(EvalAltResult::ErrorRuntime(
            format!("json_stringify from_dynamic: {e}").into(),
            rhai::Position::NONE,
        ))
    })?;
    serde_json::to_string_pretty(&val).map_err(|e| {
        Box::new(EvalAltResult::ErrorRuntime(
            format!("json_stringify: {e}").into(),
            rhai::Position::NONE,
        ))
    })
}

fn configure_engine(engine: &mut Engine) {
    engine.set_max_operations(500_000);
    engine.set_max_call_levels(50);
    engine.set_max_string_size(10_000_000);
}

pub fn create() -> Engine {
    let mut engine = Engine::new();
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .expect("Failed to create HTTP client");

    let client = std::sync::Arc::new(client);
    let c1 = client.clone();
    let c2 = client.clone();

    engine.register_fn(
        "http_get",
        move |url: &str| -> Result<String, Box<EvalAltResult>> { http_get_with_client(&c1, url) },
    );
    engine.register_fn(
        "http_get_json",
        move |url: &str| -> Result<Dynamic, Box<EvalAltResult>> {
            http_get_json_with_client(&c2, url)
        },
    );

    engine.register_fn("json_parse", json_parse);
    engine.register_fn("json_stringify", json_stringify);

    configure_engine(&mut engine);
    engine
}

pub fn create_with_client(client: reqwest::blocking::Client) -> Engine {
    let mut engine = Engine::new();

    let client = std::sync::Arc::new(client);
    let c1 = client.clone();
    let c2 = client.clone();

    engine.register_fn(
        "http_get",
        move |url: &str| -> Result<String, Box<EvalAltResult>> { http_get_with_client(&c1, url) },
    );
    engine.register_fn(
        "http_get_json",
        move |url: &str| -> Result<Dynamic, Box<EvalAltResult>> {
            http_get_json_with_client(&c2, url)
        },
    );

    engine.register_fn("json_parse", json_parse);
    engine.register_fn("json_stringify", json_stringify);

    configure_engine(&mut engine);
    engine
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::discovery::types::{DiscoveryResult, StationData, StreamUrlInfo};

    #[test]
    fn test_engine_create() {
        let engine = create();
        let ast = engine.compile("let x = 42; x").expect("compile");
        let result = engine.eval_ast::<i64>(&ast).expect("eval");
        assert_eq!(result, 42);
    }

    #[test]
    fn test_json_roundtrip() {
        let engine = create();
        let script = r#"
            let val = json_parse(`{"a": 1, "b": "hello"}`);
            json_stringify(val)
        "#;
        let result = engine
            .eval_ast::<String>(&engine.compile(script).expect("compile"))
            .expect("eval");
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["a"], 1);
        assert_eq!(parsed["b"], "hello");
    }

    #[test]
    fn test_json_array_roundtrip() {
        let engine = create();
        let script = r#"
            let val = json_parse(`[1, 2, 3]`);
            json_stringify(val)
        "#;
        let result = engine
            .eval_ast::<String>(&engine.compile(script).expect("compile"))
            .expect("eval");
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed, serde_json::json!([1, 2, 3]));
    }

    #[test]
    fn test_json_parse_error() {
        let engine = create();
        let script = r#"json_parse(`not json`)"#;
        let result = engine.eval_ast::<Dynamic>(&engine.compile(script).expect("compile"));
        assert!(result.is_err(), "expected parse error");
    }

    #[test]
    fn test_sandbox_infinite_loop() {
        let engine = create();
        let script = "loop {}";
        let ast = engine.compile(script).expect("compile");
        let result = engine.eval_ast::<Dynamic>(&ast);
        assert!(result.is_err(), "expected sandbox kill");
    }

    #[test]
    fn test_sandbox_max_operations() {
        let engine = create();
        let script = r#"
            let i = 0;
            while i < 1_000_000 {
                i += 1;
            }
            i
        "#;
        let ast = engine.compile(script).expect("compile");
        let result = engine.eval_ast::<i64>(&ast);
        assert!(result.is_err(), "expected max operations exceeded");
    }

    #[test]
    fn test_station_data_serde() {
        let station = StationData {
            name: "Test Station".into(),
            stream_url: "https://example.com/stream".into(),
            stream_urls: vec![StreamUrlInfo {
                url: "https://example.com/stream".into(),
                codec: Some("MP3".into()),
                bitrate: Some(256),
                tls: Some(true),
            }],
            homepage: Some("https://example.com".into()),
            icon_url: None,
            tags: Some("test,music".into()),
            country: Some("US".into()),
            language: Some("en".into()),
        };
        let json = serde_json::to_string(&station).unwrap();
        let back: StationData = serde_json::from_str(&json).unwrap();
        assert_eq!(back.name, "Test Station");
        assert_eq!(back.stream_urls.len(), 1);
        assert_eq!(back.stream_urls[0].bitrate, Some(256));
        assert!(back.icon_url.is_none());
    }

    #[test]
    fn test_discovery_result_serde() {
        let result = DiscoveryResult {
            provider: "Test".into(),
            stations: vec![
                StationData {
                    name: "S1".into(),
                    stream_url: "http://s1".into(),
                    stream_urls: vec![],
                    homepage: None,
                    icon_url: None,
                    tags: None,
                    country: None,
                    language: None,
                },
                StationData {
                    name: "S2".into(),
                    stream_url: "http://s2".into(),
                    stream_urls: vec![],
                    homepage: None,
                    icon_url: None,
                    tags: None,
                    country: None,
                    language: None,
                },
            ],
        };
        let json = serde_json::to_string_pretty(&result).unwrap();
        let back: DiscoveryResult = serde_json::from_str(&json).unwrap();
        assert_eq!(back.provider, "Test");
        assert_eq!(back.stations.len(), 2);
        assert_eq!(back.stations[1].name, "S2");
    }

    #[test]
    fn test_json_stringify_rhai_map() {
        let engine = create();
        let script = r#"
            let m = #{ "x": 10, "y": 20 };
            json_stringify(m)
        "#;
        let result = engine
            .eval_ast::<String>(&engine.compile(script).expect("compile"))
            .expect("eval");
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["x"], 10);
        assert_eq!(parsed["y"], 20);
    }
}
