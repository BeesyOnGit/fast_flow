use serde::de::DeserializeOwned;
use std::{
    fs::{self, OpenOptions, create_dir_all, read_to_string},
    io::{Read, Write},
    os::unix::ffi::OsStrExt,
    path::Path,
    process::Command,
};
use sysinfo::System;

use super::structs::SysInfo;

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

pub fn read_from_file_ut(file_path: &String) -> Result<String, String> {
    match read_to_string(file_path) {
        Ok(f) => return Ok(f),
        Err(err) => {
            return Err(err.to_string());
        }
    };
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
