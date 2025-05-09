use std::{thread, time::Duration};

use daemonize::Daemonize;
use fern::Dispatch;
use log::info;

use super::utils::{execute_commande, read_from_file_ut};

pub fn daemonizer(
    name: String,
    work_dir: &str,
    config_file_path: &str,
    pid_file_path: &str,
    log_file_path: &str,
    cb: fn(&str, &str) -> (),
) -> () {
    match read_from_file_ut(pid_file_path) {
        Ok(pid) => {
            let _ = execute_commande(&format!("kill {}", pid.trim()));
        }
        Err(_) => {}
    }

    // Daemonize to detach from the terminal and run in the background
    let daemonize = Daemonize::new()
        .pid_file(pid_file_path) // Prevent multiple instances
        .chown_pid_file(true) // Allow writing to the PID file
        .working_directory(".")
        .stdout(fern::log_file(&log_file_path).unwrap()) // Redirect stdout to log
        .stderr(fern::log_file(&log_file_path).unwrap()); // Redirect stderr to log
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
                .chain(fern::log_file(log_file_path).expect("Failed to create log file"))
                .apply()
                .expect("Failed to initialize logger");

            info!("<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<");
            info!("Started New instance watching [{}] ", &name);
            info!("<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<");
            loop {
                // Run the flow
                let _ = cb(work_dir, config_file_path);
                // Wait for 5 second
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
