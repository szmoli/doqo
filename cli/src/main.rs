use std::{
    fs::{self},
    path::PathBuf,
};

use common::Session;
use rust::plugin::RustPlugin; // manual imports for now
use typescript::plugin::TypeScriptPlugin;

use anyhow::{Context, Ok, Result};
use clap::Parser;
use rust_embed::RustEmbed;
use warp::Filter;

#[derive(RustEmbed)]
#[folder = "../frontend/build"]
struct Frontend;

#[derive(Parser, Debug)]
#[command(
  name = "doqo",
  version,
  about = "Generates static HTML documentation from your code symbols.",
  long_about = None
)]
struct Cli {
    #[arg(short, long, value_name = "DIR", default_value = ".")]
    pub input: PathBuf,
    #[arg(short, long, value_name = "DIR", default_value = "./docs")]
    pub output: PathBuf,
    #[arg(short = 'n', long, value_name = "PATTERN")]
    pub ignore: Vec<String>,
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
    #[arg(short, long)]
    pub serve: bool,
    #[arg(short, long, default_value = "8080")]
    pub port: u16,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let log_level = match cli.verbose {
        0 => log::LevelFilter::Warn,
        1 => log::LevelFilter::Info,
        2 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };

    env_logger::Builder::new()
        .filter_level(log_level)
        .format_target(false)
        .init();

    log::debug!("Input directory: {:?}", cli.input);
    log::debug!("Output directory: {:?}", cli.output);
    log::debug!("Ignore patterns: {:?}", cli.ignore);

    let mut session =
        Session::new(cli.input, &cli.ignore).context("Failed to initialize doqo session.")?;

    let _rust_plugin_id = session.register_plugin(Box::new(RustPlugin)); // manual registering for now
    let _typescript_plugin_id = session.register_plugin(Box::new(TypeScriptPlugin));

    session.scan_sources().context("Failed to scan sources.")?;

    let json = session.process();
    log::debug!("Registry JSON: \n{}", json);

    generate_static_site(&cli.output, &json).context("Failed to generate static site.")?;

    if cli.serve {
      serve_docs(&cli.output, cli.port).context("Failed to serve documentation.")?
    }

    Ok(())
}

fn generate_static_site(output_directory: &PathBuf, registry_json: &str) -> Result<()> {
    fs::create_dir_all(output_directory)?;

    for file in Frontend::iter() {
        let file_path = output_directory.join(file.as_ref());

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = Frontend::get(file.as_ref()).unwrap();
        fs::write(file_path, content.data)?;
    }

    let json_path = output_directory.join("registry.json");
    fs::write(json_path, registry_json)?;

    log::info!("Generated static site to {}", output_directory.display());

    Ok(())
}

#[tokio::main]
async fn serve_docs(output_directory: &PathBuf, port: u16) -> Result<()> {
    let target_directory = output_directory.clone();

    let static_files = warp::fs::dir(target_directory.clone());

    let index_fallback = warp::get().and(warp::fs::file(target_directory.join("index.html")));

    let routes = static_files.or(index_fallback);

    let addr = ([127, 0, 0, 1], port);
    let url = format!("http://localhost:{}", port);

    log::info!("Serving documentation at: {}", url);

    open::that(&url).context("Failed to open browser.")?;

    warp::serve(routes).run(addr).await;

    Ok(())
}
