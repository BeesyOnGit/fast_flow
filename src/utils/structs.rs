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
    /// Starts tracking all the configured repositorys
    Run,
    /// Stops the execution of all the configures repositorys  
    Stop,
}

#[derive(Args)]
pub struct ConfigArgs {
    // #[arg(short, long, default_value = "default")]
    /// name to use for the config file
    #[arg(short, long)]
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct ConfigFile {
    pub repo: String,
    pub build: Vec<String>,
    pub mouve: Vec<FromTo>,
    pub branche: Option<String>,
    pub version: Option<String>,
}
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct FromTo {
    pub from: String,
    pub to: String,
}
