mod utils;

use clap::Parser;
use utils::{
    structs::{Cli, Commands},
    subcommands::{init_config, run_flow, show_logs, show_status, stop_all_track},
};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let app_name = format!("fast_flow");
    let config_dir_path = format!("/etc/{}/config", &app_name);
    let work_dir = format!("/etc/{}/tmp", &app_name);
    let process_dir = format!("/etc/{}/process", &app_name);
    let logs_dir = format!("/etc/{}/logs", &app_name);

    match cli.command {
        Commands::Config(args) => init_config(args.name, &config_dir_path),
        Commands::Run(args) => run_flow(
            &work_dir,
            &process_dir,
            &logs_dir,
            &config_dir_path,
            args.name,
        ),
        Commands::Stop(args) => stop_all_track(&process_dir, args.name, false),
        Commands::Status => show_status(&process_dir, &logs_dir, &config_dir_path),
        Commands::Log(args) => show_logs(&logs_dir, args.name),
    }
}
