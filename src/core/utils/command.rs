use crate::utils::structs::SysInfo;
use log::{error, info};
use std::{
    io::{self},
    process::Command,
};
use sysinfo::System;

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
