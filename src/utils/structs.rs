use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(name = "watcher")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Creates a new config file boiler plate
    Config(ConfigArgs),
    /// Print run
    Run,
    /// watches a github repository and pull as soon as there is a change
    Watch(ConfigArgs),
}

#[derive(Args)]
pub struct ConfigArgs {
    // #[arg(short, long, default_value = "default")]
    /// name to use for the config file
    #[arg(short, long)]
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ConfigFile {
    pub repo: String,
    pub build: Vec<String>,
    pub mouve: Vec<String>,
}
