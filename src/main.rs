mod utils;

use clap::Parser;
use utils::{
    structs::{Cli, Commands},
    subcommands::{init_config, watch_config_repo},
};
#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let config_path = format!("/etc/rustflow/config");
    let work_dir = format!("/etc/rustflow/tmp");

    match cli.command {
        Commands::Config(args) => init_config(args.name, &config_path),
        Commands::Run => println!("this is run"),
        Commands::Watch(args) => watch_config_repo(args.name, &work_dir, &config_path).await,
    }
}
