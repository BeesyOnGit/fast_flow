use log::{error, info};
use serde::de::DeserializeOwned;
use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions, create_dir_all, read_to_string},
    io::{self, Read, Write},
    os::unix::ffi::OsStrExt,
    path::Path,
    process::Command,
};
use sysinfo::System;

use crate::utils::structs::ConfigFile;

use super::structs::{FromTo, SysInfo};

pub fn write_to_file_ut(file_path: &str, content: &str) -> Result<bool, String> {
    // Create all directories in the path if they don't exist
    check_dir_exist_or_create(&file_path);

    // Open or create the file
    let mut file = match OpenOptions::new().create(true).append(true).open(file_path) {
        Ok(f) => f,
        Err(err) => return Err(format!("Failed to open file: {}", err)),
    };

    // Write the content
    match file.write_all(content.as_bytes()) {
        Ok(_) => Ok(true),
        Err(err) => Err(format!("Failed to write to file: {}", err)),
    }
}

pub fn read_from_file_ut(file_path: &str) -> Result<String, String> {
    match read_to_string(file_path) {
        Ok(f) => return Ok(f),
        Err(err) => {
            return Err(err.to_string());
        }
    };
}

pub fn prompt_user(str: &str) -> Result<String, String> {
    // Prompt the user for input
    println!("{} : ", str);

    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_ret) => (),
        Err(err) => {
            return Err(format!("{}", err));
        }
    }

    let input = String::from(input.trim().to_lowercase());
    return Ok(input);
}

pub fn check_dir_exist_or_create(file_path: &str) -> () {
    let tmp_path = format!("{}", file_path);
    // Convert the file path to a Path
    let path = Path::new::<String>(&tmp_path);

    // Create all directories in the path if they don't exist
    if let Some(parent) = path.parent() {
        if let Err(err) = create_dir_all(parent) {
            println!("Failed to create directories: {}", err);
        }
    }
}

pub fn execute_commande(commande: &str) -> Result<String, String> {
    match Command::new("sh").arg("-c").arg(commande).output() {
        Ok(output) => {
            if !output.status.success() {
                // let y = String::
                return Err(String::from_utf8_lossy(&output.stderr).to_string());
            }
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        }
        Err(err) => Err(err.to_string()),
    }
}

pub fn load_file_parsed<T>(config_path: &str) -> Result<T, String>
where
    T: DeserializeOwned,
{
    // reading the config file to a string
    let file_string = match fs::read_to_string(config_path) {
        Ok(file) => file,
        Err(err) => {
            return Err(err.to_string());
        }
    };

    // serializing the config string to a struct
    match serde_json::from_str::<T>(&file_string) {
        Ok(config) => {
            return Ok(config);
        }
        Err(err) => {
            return Err(err.to_string());
        }
    };
}

pub fn extract_repo_info(url: &str) -> Option<(&str, &str)> {
    let parts: Vec<&str> = url.split('/').collect();

    // We need at least username and repo parts
    if parts.len() < 2 {
        return None;
    }

    // Get the last part (repo) and strip .git suffix
    let repo = parts.last()?.strip_suffix(".git")?;

    // The username should be second-to-last for standard GitHub URLs
    // Handle cases like "https://github.com/owner/repo.git"
    let username = parts.get(parts.len() - 2)?;

    Some((username, repo))
}

pub fn list_dir_contents(path: &str) -> Result<Vec<String>, bool> {
    let dir_content = match fs::read_dir(path) {
        Ok(content) => content,
        Err(err) => {
            print!("{}", err.to_string());
            return Err(false);
        }
    };

    let mut content = Vec::<String>::new();

    for entry in dir_content {
        let curr_entry = match entry {
            Ok(curr_entry) => curr_entry,
            Err(err) => {
                print!("{}", err.to_string());
                continue;
            }
        };

        let path = curr_entry.path();

        if path.is_dir() {
            continue;
        }

        let y = match path.file_name() {
            Some(f_name) => f_name,
            None => {
                continue;
            }
        };
        let mut str_file = String::new();

        let _ = y.as_bytes().read_to_string(&mut str_file);

        content.push(str_file);
    }
    return Ok(content);
}

pub fn get_sys_info(pid_str: &str) -> Result<SysInfo, bool> {
    let pid: usize = match pid_str.parse() {
        Ok(num) => num,
        Err(err) => {
            println!("error : {}", err);
            return Err(false);
        }
    };
    let mut system = System::new_all();
    system.refresh_all();

    match system.process(pid.into()) {
        Some(process) => {
            let mem: f32 = process.memory() as f32 / 1_000_000 as f32;
            return Ok(SysInfo {
                name: format!("{:?}", process.name()),
                cpu_usage: format!("{}%", process.cpu_usage()),
                memory: format!("{:.2}mb", mem),
                status: format!("{}", process.status()),
            });
        }
        None => Err(false),
    }

    // println!("Name: {}", process.name());
    // println!("CPU usage: {}%", process.cpu_usage());
    // println!("Memory: {} KB", process.memory());
    // println!("Status: {:?}", process.status());
}

pub fn watch_config_repo(work_dir: &str, config_file_path: &str) -> () {
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
        entry_point: _,
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

pub fn check_existing_runner(extention: &str) -> Result<&str, String> {
    use std::collections::HashMap;

    // (file-extension, (runner, args-to-print-version))
    let runners: HashMap<&'static str, (&'static str, &'static [&'static str; 1])> = [
        ("js", ("node", &["--version"])),    // Node.js
        ("ts", ("ts-node", &["--version"])), // TypeScript runner
        ("py", ("python3", &["--version"])), // Python 3+
        ("sh", ("bash", &["--version"])),    // GNU bash or compatible
        ("rb", ("ruby", &["--version"])),    // Ruby
        ("pl", ("perl", &["-v"])),           // Perl
        ("php", ("php", &["--version"])),    // PHP CLI
        ("lua", ("lua", &["-v"])),           // Lua
        ("go", ("go", &["version"])),        // Go toolchain
        ("java", ("java", &["-version"])),   // Java Runtime
    ]
    .into_iter()
    .collect();

    let (cmd, args) = runners[extention];

    // for (ext, (cmd, args)) in &runners {
    match Command::new(cmd).args(*args).status() {
        Ok(status) if status.success() => {
            return Ok(cmd);
            // println!("✔ `{}` runner for .{} is installed", cmd, ext);
        }
        Err(_err) => {
            eprintln!(
                "✖ `{}` (for .{}) not found or returned error",
                cmd, &extention
            );
            eprintln!(
                "Please install `{}` to be able to run .{} file",
                cmd, &extention
            );
            return Err(format!(
                "Please install `{}` to be able to run .{} file",
                cmd, &extention
            ));
        }
        _ => {
            return Err(format!(
                "✖ [{}] (for .{}) not found or returned error \n Please install [{}] to be able to run .{} file",
                cmd, &extention, cmd, &extention
            ));
        }
    }
    // }
}

pub fn get_process_runner(entry_point: &str) -> Result<String, String> {
    if !entry_point.contains(".") {
        return Ok("bash".to_string());
    }

    let entry_split: Vec<&str> = entry_point.split(".").collect();

    let extention = entry_split.last();

    match check_existing_runner(extention.unwrap()) {
        Ok(cm) => return Ok(cm.to_string()),
        Err(er) => {
            return Err(er);
        }
    };
}

pub fn run_binary() -> () {}

pub fn check_or_create_entry_point(
    config_file_path: &str,
    config: &mut ConfigFile,
    name: &str,
) -> Result<bool, String> {
    for target in config.mouve.clone() {
        let FromTo { from: _, mut to } = target;

        if config.entry_point.is_none() {
            config.entry_point = Some(Vec::new());
        }

        let mut already_setup = false;

        for entry_p in config.entry_point.as_mut().unwrap() {
            if entry_p.is_some() && entry_p.clone().unwrap().contains(&to.clone()) {
                already_setup = true;
                break;
            }
        }

        if already_setup {
            continue;
        }

        loop {
            let entry = match prompt_user(&format!(
                "Please insert your entry point name for the app [{name}] located at the folder {to}"
            )) {
                Ok(res) => Some(res),
                Err(err) => {
                    println!("{}", err);
                    Some(format!(""))
                }
            };
            let unwraped_entry = entry.unwrap();

            if unwraped_entry.trim() == "" {
                println!("Please insert a valid name, not an empty string");
                continue;
            }
            println!("{}/{}", &to, &unwraped_entry);
            let is_dir = match is_directory(&format!("{}/{}", &to, &unwraped_entry)) {
                Ok(res) => res,
                Err(err) => {
                    println!("{err}");
                    continue;
                }
            };

            if is_dir {
                println!(
                    "The entry point you provided is a directory, Please provide the name of a valid file or binary"
                );
                continue;
            }

            // let y = to.chars().nth(n).unwrap();
            if to.chars().nth(to.len() - 1).unwrap() == '/' {
                to.pop();
            };

            let entry_string = format!("{}/{}", &to, &unwraped_entry);

            config
                .entry_point
                .as_mut()
                .unwrap()
                .push(Some(entry_string));

            break;
        }
    }
    match execute_commande(&format!("rm {}", &config_file_path)) {
        Ok(_) => {
            let _ = write_to_file_ut(
                &config_file_path,
                &serde_json::to_string_pretty::<ConfigFile>(&config).unwrap(),
            );
            return Ok(true);
        }
        Err(err) => {
            return Err(err);
        }
    }
}

pub fn is_directory(path_to_check: &str) -> Result<bool, String> {
    let metadata = match fs::metadata(path_to_check) {
        Ok(data) => data,
        Err(err) => {
            return Err(err.to_string());
        }
    };

    return Ok(metadata.is_dir());
}

fn load_env_file(path: &str) -> Result<HashMap<String, String>, String> {
    let mut vars = HashMap::new();

    let contents = match fs::read_to_string(path) {
        Ok(cont) => cont,
        Err(err) => {
            return Err(err.to_string());
        }
    };

    for line in contents.lines() {
        if line.starts_with('#') && !line.contains('=') {
            continue;
        }
        let parts: Vec<_> = line.splitn(2, '=').collect();
        if parts.len() == 2 {
            vars.insert(parts[0].trim().to_string(), parts[1].trim().to_string());
        }
    }

    return Ok(vars);
}
