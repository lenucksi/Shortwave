use rhai::{Dynamic, Engine, EvalAltResult};
use scraper::{Html, Selector};
use serde_json::{json, Value};
use std::fs;

fn http_get(url: &str) -> Result<String, Box<EvalAltResult>> {
    let resp = reqwest::blocking::get(url).map_err( | e | {
        Box::new(EvalAltResult::ErrorRuntime(
            format!("http_get: {}", e).into(),
            rhai::Position::NONE,
        ))
    })?;
    resp.text().map_err( | e | {
        Box::new(EvalAltResult::ErrorRuntime(
            format!("http_get body: {}", e).into(),
            rhai::Position::NONE,
        ))
    })
}

fn http_get_json(url: &str) -> Result<Dynamic, Box<EvalAltResult>> {
    let body = http_get(url)?;
    let val: Value = serde_json::from_str(&body).map_err( | e | {
        Box::new(EvalAltResult::ErrorRuntime(
            format!("json parse: {}", e).into(),
            rhai::Position::NONE,
        ))
    })?;
    rhai::serde::to_dynamic(&val).map_err( | e | {
        Box::new(EvalAltResult::ErrorRuntime(
            format!("to_dynamic: {}", e).into(),
            rhai::Position::NONE,
        ))
    })
}

fn json_parse(s: &str) -> Result<Dynamic, Box<EvalAltResult>> {
    let val: Value = serde_json::from_str(s).map_err( | e | {
        Box::new(EvalAltResult::ErrorRuntime(
            format!("json_parse: {}", e).into(),
            rhai::Position::NONE,
        ))
    })?;
    rhai::serde::to_dynamic(&val).map_err( | e | {
        Box::new(EvalAltResult::ErrorRuntime(
            format!("to_dynamic: {}", e).into(),
            rhai::Position::NONE,
        ))
    })
}

fn json_stringify(val: Dynamic) -> Result<String, Box<EvalAltResult>> {
    let val: Value = rhai::serde::from_dynamic(&val).map_err( | e | {
        Box::new(EvalAltResult::ErrorRuntime(
            format!("from_dynamic: {}", e).into(),
            rhai::Position::NONE,
        ))
    })?;
    serde_json::to_string_pretty(&val).map_err( | e | {
        Box::new(EvalAltResult::ErrorRuntime(
            format!("json_stringify: {}", e).into(),
            rhai::Position::NONE,
        ))
    })
}

fn html_select(html_str: &str, css: &str) -> Result<String, Box<EvalAltResult>> {
    let html = Html::parse_document(html_str);
    let sel = Selector::parse(css).map_err( | e | {
        Box::new(EvalAltResult::ErrorRuntime(
            format!("invalid css '{}': {}", css, e).into(),
            rhai::Position::NONE,
        ))
    })?;

    let results: Vec<Value> = html.select(&sel).map( | el | {
        let mut attrs = serde_json::Map::new();
        for (k, v) in el.value().attrs() {
            attrs.insert(k.to_string(), Value::String(v.to_string()));
        }
        json!({
            "tag": el.value().name(),
            "attrs": attrs,
            "text": el.text().collect::<Vec<_>>().join(""),
            "html": el.html(),
        })
    }).collect();

    serde_json::to_string(&results).map_err( | e | {
        Box::new(EvalAltResult::ErrorRuntime(
            format!("serialize: {}", e).into(),
            rhai::Position::NONE,
        ))
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: rhai-runner <script.rhai>");
        std::process::exit(1);
    }

    let script = fs::read_to_string(&args[1])
        .map_err( | e | format!("read '{}': {}", args[1], e))?;

    let mut engine = Engine::new();
    engine.register_fn("http_get", http_get);
    engine.register_fn("http_get_json", http_get_json);
    engine.register_fn("json_parse", json_parse);
    engine.register_fn("json_stringify", json_stringify);
    engine.register_fn("html_select", html_select);
    engine.set_max_operations(1_000_000);
    engine.set_max_call_levels(50);
    engine.set_max_string_size(10_000_000);

    let ast = engine.compile(&script)
        .map_err( | e | format!("compile error: {}", e))?;
    let result = engine.eval_ast::<Dynamic>(&ast)
        .map_err( | e | format!("runtime error: {}", e))?;

    println!("{}", result);

    Ok(())
}
