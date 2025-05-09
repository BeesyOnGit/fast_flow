use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Parser)]
#[command(
    name = "flow",
    about = "A tool for monitoring and managing repository changes"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new configuration file with boilerplate structure
    Config(ConfigArgs),

    /// Start tracking all configured repositories for changes
    Watch(OptConfigArgs),

    /// Stop monitoring of specified or all repositories
    Stop(OptConfigArgs),

    /// Display current status of watched repositories in table format
    Status,

    /// Display the logs of the selected tracked repository
    Log(ConfigArgs),

    /// Start the execution of the selected application in a new process
    Start(OptConfigArgs),
}

#[derive(Args)]
pub struct ConfigArgs {
    /// Specify a name for the configuration file
    #[arg(short, long, help = "Name of the configuration file to create")]
    pub name: String,
}

#[derive(Args)]
pub struct OptConfigArgs {
    /// Name of the specific process to stop (omit for all processes)
    #[arg(
        short,
        long,
        help = "Optional: Name of specific repository process to stop"
    )]
    pub name: Option<String>,
}

#[derive(Args)]
pub struct StatusArgs {
    /// Show only repositories being actively watched
    #[arg(short, long, help = "Filter to show only watched repositories")]
    pub watch: bool,

    /// Show detailed process information
    #[arg(short, long, help = "Include detailed process statistics")]
    pub process: bool,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct ConfigFile {
    pub repo: String,
    pub build: Vec<String>,
    pub mouve: Vec<FromTo>,
    pub branch: Option<String>,
    pub version: Option<String>,
    pub entry_point: Option<String>,
}
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct FromTo {
    pub from: String,
    pub to: String,
}
#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct WatchStats {
    pub pid: String,
    pub name: String,
    pub repo: String,
    pub branch: String,
    pub cpu: String,
    pub memory: String,
    pub status: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SysInfo {
    pub name: String,
    pub cpu_usage: String,
    pub memory: String,
    pub status: String,
}
