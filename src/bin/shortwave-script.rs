use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

use shortwave::discovery::engine::create;
use shortwave::discovery::provider::DiscoveryProvider;
use shortwave::discovery::registry::ProviderRegistry;
use shortwave::discovery::runner::{RunnerError, run_provider};
use shortwave::discovery::types::DiscoveryResult;

#[derive(Parser)]
#[command(name = "shortwave-script", about = "Shortwave Discovery Script Runner")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a provider script and print results
    Run {
        /// Path to .rhai script
        path: PathBuf,
        /// Human-readable station output instead of JSON
        #[arg(long, short)]
        pretty: bool,
        /// Script timeout in seconds
        #[arg(long, default_value = "30")]
        timeout: u64,
    },
    /// List available provider scripts
    List,
    /// Validate a provider script
    Validate {
        /// Path to .rhai script
        path: PathBuf,
        /// Strict schema validation
        #[arg(long)]
        strict: bool,
    },
}

fn print_pretty(result: &DiscoveryResult) {
    println!("Provider: {}", result.provider);
    println!("Stations: {}", result.stations.len());
    println!();
    for (i, station) in result.stations.iter().enumerate() {
        println!("{}. {}", i + 1, station.name);
        println!("   URL: {}", station.stream_url);
        for url in &station.stream_urls {
            let tls = if url.tls.unwrap_or(false) {
                "TLS"
            } else {
                "no TLS"
            };
            let codec = url.codec.as_deref().unwrap_or("?");
            let bitrate = url
                .bitrate
                .map(|b| format!("{b} kbps"))
                .unwrap_or_else(|| "?".to_string());
            println!("   {} | {} | {}", codec, bitrate, tls);
        }
        if let Some(homepage) = &station.homepage {
            println!("   Web: {}", homepage);
        }
        if let Some(tags) = &station.tags {
            println!("   Tags: {}", tags);
        }
        if let Some(country) = &station.country {
            println!("   Country: {}", country);
        }
        println!();
    }
}

fn cmd_run(path: &Path, pretty: bool, _timeout: u64) -> Result<(), RunnerError> {
    let engine = create();
    let provider = DiscoveryProvider {
        id: String::new(),
        name: String::new(),
        description: String::new(),
        script_path: path.to_path_buf(),
        enabled: true,
    };
    let result = run_provider(&engine, &provider)?;
    if pretty {
        print_pretty(&result);
    } else {
        println!("{}", serde_json::to_string_pretty(&result).unwrap());
    }
    Ok(())
}

fn cmd_list() {
    let registry = ProviderRegistry::scan();
    let providers = registry.providers();
    if providers.is_empty() {
        println!("No provider scripts found.");
        return;
    }
    for p in providers {
        let status = if p.enabled { "enabled" } else { "disabled" };
        println!("  {} ({})", p.name, p.id);
        if !p.description.is_empty() {
            println!("    {}", p.description);
        }
        println!("    Path: {}", p.script_path.display());
        println!("    Status: {}", status);
    }
}

fn cmd_validate(path: &Path, _strict: bool) -> Result<(), RunnerError> {
    let engine = create();
    let provider = DiscoveryProvider {
        id: String::new(),
        name: String::new(),
        description: String::new(),
        script_path: path.to_path_buf(),
        enabled: true,
    };
    match run_provider(&engine, &provider) {
        Ok(result) => {
            println!("✓ Valid script — {} stations found", result.stations.len());
            Ok(())
        }
        Err(e) => {
            eprintln!("✗ Validation failed: {e}");
            std::process::exit(1);
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Commands::Run {
            path,
            pretty,
            timeout,
        } => cmd_run(&path, pretty, timeout),
        Commands::List => {
            cmd_list();
            Ok(())
        }
        Commands::Validate { path, strict } => cmd_validate(&path, strict),
    };
    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
