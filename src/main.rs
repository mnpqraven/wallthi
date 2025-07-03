use crate::{
    command::{
        Commands, run_daemon,
        status::daemon_status,
        swww_control::{pause_all, start_or_resume},
    },
    dot_config::{DotfileTreeConfig, read_config},
    utils::error::AppError,
};
use clap::Parser;
use std::path::PathBuf;

mod command;
mod dot_config;
mod utils;

#[derive(Parser, Debug)]
#[command(version, about)]
struct CliArgs {
    /// toml config file
    #[arg(long)]
    config: Option<PathBuf>,
    #[command(subcommand)]
    command: Commands,
}

fn main() -> Result<(), AppError> {
    let args = CliArgs::parse();
    let conf = match args.config {
        Some(conf_path) => read_config(conf_path)?,
        None => DotfileTreeConfig::default(),
    };
    println!("{conf:?}");

    match args.command {
        Commands::Daemon => run_daemon(),
        Commands::Status => {
            let status = daemon_status()?;
            println!("{status:?}");
        }
        Commands::Pause => pause_all(),
        Commands::Start => start_or_resume(),
    };

    Ok(())
}
