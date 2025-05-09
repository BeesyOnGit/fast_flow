use crate::utils::{
    content::config_example, structs::ConfigFile, table::watch_status_table,
    utils::load_file_parsed,
};
use std::{thread, time::Duration};
use tokio::task;

use super::{
    daemon::daemonizer,
    structs::{SysInfo, WatchStats},
    utils::{
        check_dir_exist_or_create, execute_commande, extract_repo_info, get_sys_info,
        list_dir_contents, read_from_file_ut, start_process, watch_config_repo, write_to_file_ut,
    },
};

pub fn init_config(name: String, path: &str) -> () {
    let config_path = format!("{path}/{name}.config.json");

    match read_from_file_ut(&config_path) {
        Ok(_) => {
            println!("Error! the specified name already has a config check at {config_path}");
            return;
        }
        Err(_err) => {}
    };

    match write_to_file_ut(&config_path, &config_example()) {
        Ok(_) => (),
        Err(err) => {
            println!("{}", err);
        }
    };

    println!("config file boiler plate created go edit it at {config_path} ")
}

pub fn watch_repo(
    work_dir: &str,
    process_dir: &str,
    logs_dir: &str,
    config_dir_path: &str,
    name: Option<String>,
) -> () {
    let _ = check_dir_exist_or_create(&format!("{}/example", config_dir_path));
    let _ = check_dir_exist_or_create(&format!("{}/example", work_dir));
    let _ = check_dir_exist_or_create(&format!("{}/example", process_dir));
    let _ = check_dir_exist_or_create(&format!("{}/example", logs_dir));

    let mut liste: Vec<String> = Vec::new();

    if name.is_none() {
        liste = match list_dir_contents(&config_dir_path) {
            Ok(content) => content,
            Err(err) => {
                print!("{}", err.to_string());
                return;
            }
        }
    }

    if name.is_some() {
        liste.push(format!("{}.config.json", name.unwrap()));
    }

    let mut names = Vec::<String>::new();

    for elem in liste {
        let r: Vec<&str> = elem.split(".").collect();
        names.push(r[0].to_string());
    }

    let work_dir = work_dir.to_owned();
    let process_dir = process_dir.to_owned();
    let logs_dir = logs_dir.to_owned();
    let config_dir_path = config_dir_path.to_owned();

    for name in names {
        let work_dir = work_dir.clone();
        let process_dir = process_dir.clone();
        let logs_dir = logs_dir.clone();
        let config_dir_path = config_dir_path.clone();

        let config_file_path = format!("{}/{}.config.json", &config_dir_path, &name);
        let pid_file_path = format!("{}/{}.process.pid", &process_dir, &name);
        let log_file_path = format!("{}/{}.process.log", &logs_dir, &name);

        // daemonizer is fully blocking, so use the blocking thread-pool
        task::spawn_blocking(move || {
            daemonizer(
                name,
                &work_dir,
                &config_file_path,
                &pid_file_path,
                &log_file_path,
                watch_config_repo,
            );
        });
    }
}
pub fn run_flow(
    work_dir: &str,
    process_dir: &str,
    logs_dir: &str,
    config_dir_path: &str,
    name: Option<String>,
) -> () {
    let _ = check_dir_exist_or_create(&format!("{}/example", config_dir_path));
    let _ = check_dir_exist_or_create(&format!("{}/example", work_dir));
    let _ = check_dir_exist_or_create(&format!("{}/example", process_dir));
    let _ = check_dir_exist_or_create(&format!("{}/example", logs_dir));

    let mut liste: Vec<String> = Vec::new();

    if name.is_none() {
        liste = match list_dir_contents(&config_dir_path) {
            Ok(content) => content,
            Err(err) => {
                print!("{}", err.to_string());
                return;
            }
        }
    }

    if name.is_some() {
        liste.push(format!("{}.config.json", name.unwrap()));
    }

    let mut names = Vec::<String>::new();

    for elem in liste {
        let r: Vec<&str> = elem.split(".").collect();
        names.push(r[0].to_string());
    }

    let work_dir = work_dir.to_owned();
    let process_dir = process_dir.to_owned();
    let logs_dir = logs_dir.to_owned();
    let config_dir_path = config_dir_path.to_owned();

    for name in names {
        let work_dir = work_dir.clone();
        let process_dir = process_dir.clone();
        let logs_dir = logs_dir.clone();
        let config_dir_path = config_dir_path.clone();

        let config_file_path = format!("{}/{}.config.json", &config_dir_path, &name);
        let pid_file_path = format!("{}/{}.watch.pid", &process_dir, &name);
        let log_file_path = format!("{}/{}.watch.log", &logs_dir, &name);
        start_process(&config_file_path);

        // // daemonizer is fully blocking, so use the blocking thread-pool
        // task::spawn_blocking(move || {
        //     daemonizer(
        //         name,
        //         &work_dir,
        //         &config_file_path,
        //         &pid_file_path,
        //         &log_file_path,
        //         watch_config_repo,
        //     );
        // });
    }
}

pub fn stop_all_track(process_dir: &str, name: Option<String>, silent: bool) -> () {
    let mut liste: Vec<String> = Vec::new();

    if name.is_none() {
        liste = match list_dir_contents(&process_dir) {
            Ok(content) => content,
            Err(err) => {
                if silent == true {
                    return;
                }
                println!("{}", err.to_string());
                return;
            }
        };
    }

    if name.is_some() {
        liste.push(format!("{}.watch.pid", name.unwrap()));
    }

    if liste.len() == 0 {
        if silent == true {
            return;
        }

        println!("You have No process running to stop!");
        return;
    }

    if silent != true {
        println!("Shutting down {} Runing processes", &liste.len());
    }

    for elem in liste {
        let tmp_pid = match read_from_file_ut(&format!("{}/{}", &process_dir, &elem)) {
            Ok(content) => content,
            Err(err) => {
                println!("{}", err.to_string());
                continue;
            }
        };
        let _ = execute_commande(&format!("kill {}", &tmp_pid.trim()));
        let _ = execute_commande(&format!("rm -rf {}/{}", &process_dir, &elem));
    }

    if silent != true {
        println!("all your repositories tracking was terminated");
    }

    return;
}

pub fn show_logs(logs_dir: &str, name: String) -> () {
    loop {
        let file_content = match read_from_file_ut(&format!("{}/{}.watch.log", &logs_dir, &name)) {
            Ok(cont) => {
                // console.clear_screen().unwrap();
                // print!("\x1B[2J\x1B[1;1H");
                print!("{esc}c", esc = 27 as char);
                cont
            }
            Err(err) => {
                println!("{err}");
                return;
            }
        };

        // let _ = console.write_line(&format!("{}", file_content));
        println!("{file_content}");
        thread::sleep(Duration::from_secs(5));
    }
}

pub fn show_status(process_dir: &str, _logs_dir: &str, config_dir_path: &str) -> () {
    let liste = match list_dir_contents(&config_dir_path) {
        Ok(content) => content,
        Err(err) => {
            println!("{}", err.to_string());
            return;
        }
    };

    if liste.len() == 0 {
        println!("You have No Repositorys being watched right now!");
        return;
    }
    let mut data: Vec<WatchStats> = Vec::new();

    for file_name in liste {
        let config =
            match load_file_parsed::<ConfigFile>(&format!("{}/{}", &config_dir_path, &file_name)) {
                Ok(conf) => conf,
                Err(err) => {
                    println!("{}", err);
                    continue;
                }
            };

        let ConfigFile {
            repo,
            build: _,
            mouve: _,
            branch,
            version: _,
            entry_point: _,
        } = config;

        // Extract the user name and repo
        let repo_info = match extract_repo_info(&repo) {
            Some(rep) => rep,
            None => {
                println!("error while parsing your github repo to extract the name, check it");
                return;
            }
        };

        // extracting username and repository from repo string
        let (username, folder_name) = repo_info;

        let name: Vec<&str> = file_name.split(".").collect();

        let pid = match read_from_file_ut(&format!("{}/{}.watch.pid", &process_dir, &name[0])) {
            Ok(content) => content.trim().to_string(),
            Err(_err) => {
                let data_elem = WatchStats {
                    name: format!("{}", name[0]),
                    pid: "N/A".to_string(),
                    repo: format!("{username}/{folder_name}.git"),
                    branch: branch.unwrap_or("main".to_string()),
                    cpu: "N/A".to_string(),
                    memory: "N/A".to_string(),
                    status: "unwatched".to_string(),
                };

                data.push(data_elem);
                continue;
            }
        };

        let sys_info = match get_sys_info(&pid) {
            Ok(sy) => sy,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };

        let SysInfo {
            name: _,
            cpu_usage,
            memory,
            status: _,
        } = sys_info;

        let data_elem = WatchStats {
            name: format!("{}", name[0]),
            pid,
            repo: format!("{username}/{folder_name}.git"),
            branch: branch.unwrap_or("main".to_string()),
            cpu: cpu_usage,
            memory: memory,
            status: "watched".to_string(),
        };

        data.push(data_elem);
    }
    let table = watch_status_table(data, "Fast⚡Flow Watching Status");
    println!("{table}");
}
