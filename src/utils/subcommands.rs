use crate::utils::{
    content::config_example, structs::ConfigFile, table::watch_status_table,
    utils::load_file_parsed,
};
use daemonize::Daemonize;
use fern::Dispatch;
use log::{error, info};
use std::{thread, time::Duration};
use tokio::task;

use super::{
    structs::{SysInfo, WatchStats},
    utils::{
        check_dir_exist_or_create, execute_commande, extract_repo_info, get_sys_info,
        list_dir_contents, read_from_file_ut, write_to_file_ut,
    },
};

use console::Term;

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

    // let liste = match list_dir_contents(&config_dir_path) {
    //     Ok(content) => content,
    //     Err(err) => {
    //         print!("{}", err.to_string());
    //         return;
    //     }
    // };
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

    // // stop all current running processes befor starting new ones
    // let _ = stop_all_track(&process_dir, None, true);

    for name in names {
        let work_dir = work_dir.clone();
        let process_dir = process_dir.clone();
        let logs_dir = logs_dir.clone();
        let config_dir_path = config_dir_path.clone();

        // daemonizer is fully blocking, so use the blocking thread-pool
        task::spawn_blocking(move || {
            daemonizer(name, work_dir, config_dir_path, process_dir, logs_dir);
        });
    }
}

pub fn daemonizer(
    name: String,
    work_dir: String,
    config_dir_path: String,
    process_dir: String,
    logs_dir: String,
) -> () {
    // Create PID name
    let pid_file_name = &format!("{}/{}.watch.pid", &process_dir, &name);
    let log_path = format!("{}/{}.watch.log", &logs_dir, &name);

    match read_from_file_ut(&pid_file_name) {
        Ok(pid) => {
            let _ = execute_commande(&format!("kill {}", pid.trim()));
        }
        Err(_) => {}
    }

    // Daemonize to detach from the terminal and run in the background
    let daemonize = Daemonize::new()
        .pid_file(pid_file_name) // Prevent multiple instances
        .chown_pid_file(true) // Allow writing to the PID file
        .working_directory(".")
        .stdout(fern::log_file(&log_path).unwrap()) // Redirect stdout to log
        .stderr(fern::log_file(&log_path).unwrap()); // Redirect stderr to log
    // .privileged_action(|| println!("Background process started"));

    match daemonize.start() {
        Ok(_) => {
            // init loggoing
            Dispatch::new()
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "[{}][{}] {}",
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                        record.level(),
                        message
                    ))
                })
                .level(log::LevelFilter::Debug) // Adjust log level as needed
                .chain(fern::log_file(&log_path).expect("Failed to create log file"))
                .apply()
                .expect("Failed to initialize logger");

            info!("<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<");
            info!("Started New instance watching [{}] ", &name);
            info!("<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<");
            loop {
                // Run the flow
                let _ = watch_config_repo(&name, &work_dir, &config_dir_path);
                // Wait for 1 second
                thread::sleep(Duration::from_secs(5));
            }
        }
        Err(e) => eprintln!("Failed to start daemon: {}", e),
    }
}

// change the name of the daemonized process TODO!
// use libc;
// use std::ffi::CString;
// fn set_process_name(name: &str) {
//     let c_name = CString::new(name).unwrap();
//     unsafe {
//         // Linux-specific
//         libc::prctl(libc::PR_SET_NAME, c_name.as_ptr() as libc::c_ulong);

//         // BSD/MacOS alternative
//         #[cfg(target_os = "macos")]
//         libc::pthread_setname_np(c_name.as_ptr());

//         // For argv[0] (visible in ps/top)
//         libc::strncpy(
//             std::env::args().next().unwrap().as_ptr() as *mut libc::c_char,
//             c_name.as_ptr(),
//             name.len()
//         );
//     }
// }

// // Usage with daemonize:
// Daemonize::new()
//     .privileged_action(|| set_process_name("my_custom_daemon"))
//     .start();

pub fn watch_config_repo(name: &str, work_dir: &str, config_dir_path: &str) -> () {
    let config_file_path = format!("{}/{}.config.json", config_dir_path, &name);

    info!("Reading config");

    let mut config = match load_file_parsed::<ConfigFile>(&config_file_path) {
        Ok(conf) => conf,
        Err(err) => {
            error!("{}", err);
            return;
        }
    };

    let ConfigFile {
        build,
        repo,
        mouve,
        version,
        branch,
    } = config.clone();

    let curr_version = match version {
        Some(v) => v,
        None => String::new(),
    };

    let actual_branch = match branch {
        Some(b) => b,
        None => "main".to_string(),
    };

    // Check if workdir exist else create
    check_dir_exist_or_create(&format!("{}/exmaple", &work_dir));

    info!(
        "Fetching the most recent version for branch {}",
        &actual_branch
    );

    // Extract the user name and repo
    let repo_info = match extract_repo_info(&repo) {
        Some(rep) => rep,
        None => {
            error!("error while parsing your github repo to extract the name, check it");
            let _ = execute_commande(&format!("cd {} && rm -rf {}", &work_dir, &repo));
            return;
        }
    };

    // extracting username and repository from repo string
    let (username, folder_name) = repo_info;

    // remove fetched in case it exist
    let _ = execute_commande(&format!("rm -rf {}/{}", &work_dir, &folder_name));

    // Fetch the current repo version
    let fetch_version = match execute_commande(&format!(
        "git ls-remote git@github.com:{}/{}.git {:?}",
        &username, &folder_name, &actual_branch
    )) {
        Ok(v) => v.trim().split("refs").next().unwrap().to_string(),
        Err(err) => {
            error!("{}", err);
            return;
        }
    };

    // If the current version is the newest do nothing
    if curr_version == fetch_version {
        info!("Up to date with branch");
        return;
    }

    // refreshing the controle version
    config.version = Some(fetch_version);
    config.branch = Some(actual_branch);

    // Clone repository in local
    match execute_commande(&format!(
        "cd {work_dir} && git clone git@github.com:{}/{}.git",
        &username, &folder_name
    )) {
        Ok(_) => info!("cloned repository {repo}"),
        Err(err) => {
            error!("{}", err);
            return;
        }
    }

    // Executing build

    info!("Starting Building Process");

    for command in build {
        match execute_commande(&format!(
            "cd {}/{} && {}",
            &work_dir, &folder_name, &command
        )) {
            Ok(val) => {
                info!("{}", val);
                info!("{command} : commande success ")
            }
            Err(err) => {
                let _ = execute_commande(&format!("cd {} && rm -rf {}", &work_dir, &folder_name));
                error!("{}", err);
                return;
            }
        }
    }
    // Executing move

    info!("Starting Moving Process");
    for command in mouve {
        check_dir_exist_or_create(&format!("{}/example", &command.to));

        match execute_commande(&format!(
            "cd {}/{} && cp -r {} {}",
            &work_dir, &folder_name, &command.from, &command.to
        )) {
            Ok(_) => info!("moving {} : commande success ", &command.from),
            Err(err) => {
                let _ = execute_commande(&format!("cd {} && rm -rf {}", &work_dir, &folder_name));
                let _ = execute_commande(&format!("rm -rf {}", &command.to,));
                error!("{}", err);
                return;
            }
        }
    }
    let _ = execute_commande(&format!("cd {} && rm -rf {}", &work_dir, &folder_name));

    info!("repository tracked");

    match execute_commande(&format!("rm {}", &config_file_path)) {
        Ok(_) => {
            let _ = write_to_file_ut(
                &config_file_path,
                &serde_json::to_string_pretty::<ConfigFile>(&config).unwrap(),
            );
        }
        Err(err) => {
            error!("{err}");
            return;
        }
    }

    return ();
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
    let console = Term::stdout();
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

pub fn show_status(process_dir: &str, logs_dir: &str, config_dir_path: &str) -> () {
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
    let table = watch_status_table(data, "Fast Flow Watching Status");
    println!("{table}");
}
