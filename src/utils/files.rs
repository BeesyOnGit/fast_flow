use std::{fs::OpenOptions, io::Write, thread, time::Duration};

use daemonize::Daemonize;

pub fn start_background_process(num: i32) {
    println!("starting count process");
    // Daemonize to detach from the terminal and run in the background
    let daemonize = Daemonize::new()
        .pid_file("/tmp/watcher.pid") // Prevent multiple instances
        .chown_pid_file(true) // Allow writing to the PID file
        .working_directory(".") // Set working directory
        .privileged_action(|| println!("Background process started"));

    match daemonize.start() {
        Ok(_) => {
            // This block runs in the daemonized process
            let mut i = 0;
            let file_path = format!("/tmp/watcher{num}_counter.log");

            // Open the file in append mode (create if missing)
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(file_path)
                .expect("Failed to open file");
            loop {
                // Write the counter value
                file.write_all(format!("line {} \n", &i).as_bytes())
                    .expect("error writing to file");
                // writeln!(file, "{}", i).expect("Failed to write to file");
                i += 1;

                // Wait for 1 second
                thread::sleep(Duration::from_secs(1));
            }
        }
        Err(e) => eprintln!("Failed to start daemon: {}", e),
    }
}
