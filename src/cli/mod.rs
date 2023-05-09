use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

use crate::config::Config;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Default)]
pub enum Command {
    #[default]
    Path,
}

pub fn parse_cli(cli: &Cli, cfg: &Config) -> Result<()> {
    if cli.command.is_none() {
        println!("Hello, {}!", cfg.name);
        return Ok(());
    }

    match cli.command.as_ref().unwrap() {
        Command::Path => {
            println!(
                "{}",
                confy::get_configuration_file_path("tasker", "tasker_tui")?
                    .to_str()
                    .context("Path is not valid unicode")?
            );

            Ok(())
        }
    }
}
