use anyhow::Result;
use clap::Parser;

mod config;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Path to configuration file
    #[arg(short, long, default_value = "/etc/sajuuk/sajuuk.yaml")]
    config: String,

    /// Run in daemon mode
    #[arg(short, long)]
    daemon: bool,

    /// Stop running instance
    #[arg(long)]
    stop: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    // Load config
    let _config = config::load_config(&args.config)?;

    println!("Hello, world!");

    // Wait for shutdown signal
    tokio::signal::ctrl_c().await?;
    Ok(())
}
