use crate::utils::{content::config_example, structs::ConfigFile, utils::load_file_parsed};

use super::utils::{
    check_dir_exist_or_create, execute_commande, extract_repo_info, read_from_file_ut,
    write_to_file_ut,
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

    match write_to_file_ut(&config_path, config_example()) {
        Ok(_) => (),
        Err(err) => {
            println!("{}", err);
        }
    };

    println!("config file boiler plate created go edit it at {config_path} ")
}

pub async fn watch_config_repo(name: String, work_dir: &String, config_path: &str) -> () {
    let config = match load_file_parsed::<ConfigFile>(&name, config_path).await {
        Ok(conf) => conf,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };
    let ConfigFile { build, repo, mouve } = config;

    // check if workdir exist else create
    check_dir_exist_or_create(&format!("{}/exmaple", &work_dir));

    // extract the user name and repo
    let repo_info = match extract_repo_info(&repo) {
        Some(rep) => rep,
        None => {
            println!("error while parsing your github repo to extract the name, check it");
            let _ = execute_commande(&format!("cd {} && rm -rf {}", &work_dir, &repo)).await;
            return;
        }
    };
    let (username, folder_name) = repo_info;

    // clone repository in local
    match execute_commande(&format!(
        "cd {work_dir} && git clone git@github.com:{}/{}.git",
        &username, &folder_name
    ))
    .await
    {
        Ok(_) => println!("cloned repository {repo}"),
        Err(err) => {
            println!("{}", err);
            return;
        }
    }

    // Executing build

    println!("Starting Building Process");
    for command in build {
        match execute_commande(&format!(
            "cd {}/{} && {}",
            &work_dir, &folder_name, &command
        ))
        .await
        {
            Ok(_) => println!("{command} : commande success "),
            Err(err) => {
                let _ =
                    execute_commande(&format!("cd {} && rm -rf {}", &work_dir, &folder_name)).await;
                println!("{}", err);
                return;
            }
        }
    }
    // Executing move

    println!("Starting Moving Process");
    for command in mouve {
        check_dir_exist_or_create(&command.to);

        match execute_commande(&format!(
            "cd {}/{} && cp {} {}",
            &work_dir, &folder_name, &command.from, &command.to
        ))
        .await
        {
            Ok(_) => println!("moving {} : commande success ", &command.from),
            Err(err) => {
                let _ =
                    execute_commande(&format!("cd {} && rm -rf {}", &work_dir, &folder_name)).await;
                println!("{}", err);
                return;
            }
        }
    }
    let _ = execute_commande(&format!("cd {} && rm -rf {}", &work_dir, &folder_name)).await;
    return ();
}
