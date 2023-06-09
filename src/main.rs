use std::{thread, time::Duration};

use anyhow::Result;
use clap::Parser;

use tasker_tui::{
    cli::{parse_cli, Cli},
    config::load_config,
    tui::{initiate_tui, terminate_tui},
};

fn main() -> Result<()> {
    // Parse command-line arguments
    let cli = Cli::parse();

    // Load configuration file
    let cfg = load_config()?;

    // Setup for Terminal interface
    let terminal = initiate_tui()?;

    thread::sleep(Duration::from_secs(5));

    // Close the interface
    terminate_tui(terminal)?;

    parse_cli(&cli, &cfg)?;

    Ok(())
}
