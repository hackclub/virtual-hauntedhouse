use std::process::{Command, Stdio};
use std::fs;
use std::fs::File;
use std::io::{Write, Result};
use std::fs::OpenOptions;

struct EventInfo {
    repo_url: String,
    target_folder: String,
    output_file: String,
}

const EVENT_INFOS: &[EventInfo] = &[
    EventInfo {
        repo_url: "https://github.com/ivoinestrachan/haunted-house-testing".to_string(),
        target_folder: "bayarea/www".to_string(),
        output_file: "bayarea.md".to_string(),
    },
    EventInfo {
        repo_url: "https://github.com/ShubhamPatilsd/wandering-wavelength".to_string(),
        target_folder: "toronto/www".to_string(),
        output_file: "toronto.md".to_string(),
    },
];

fn main() {
    for event_info in EVENT_INFOS.iter() {
        let mut file = match OpenOptions::new().append(true).create(true).open(&event_info.output_file) {
            Ok(f) => f,
            Err(e) => {
                println!("\x1b[31m[ERROR]\x1b[0m failed to open {} file: {}", event_info.output_file, e);
                return;
            }
        };

        clone_and_deploy_website(&event_info, &mut file);
    }
}

fn clone_and_deploy_website(event_info: &EventInfo, file: &mut File) {
    let repo_url = &event_info.repo_url;
    let repo_name = repo_url.split('/').last().unwrap_or("repo");
    let repo_folder = &event_info.target_folder;

    if fs::metadata(&repo_folder).is_ok() {
        println!("\x1b[33m[SKIPPING]\x1b[0m repository {} already exists. Skipping clone.", repo_folder);
    } else {
        println!("\x1b[36m[CLONING]\x1b[0m cloning repository from {} into folder {}", repo_url, repo_folder);

        let git_clone_result = Command::new("git")
            .arg("clone")
            .arg(repo_url)
            .arg(&repo_folder)
            .output();

        match git_clone_result {
            Ok(_output) => println!("\x1b[32m[SUCCESS]\x1b[0m repository {} cloned successfully.", repo_folder),
            Err(err) => println!("\x1b[31m[ERROR]\x1b[0m failed to clone repository: {}", err),
        }

        if let Err(err) = fs::create_dir_all(&repo_folder) {
            println!("\x1b[31m[ERROR]\x1b[0m failed to create directory: {}", err);
            return;
        }

        let vercel_command = format!("cd {} && vercel --yes", repo_folder);
        let shell_result = Command::new("sh")
            .arg("-c")
            .arg(&vercel_command)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status();

        match shell_result {
            Ok(status) if status.success() => {
                println!("\x1b[32m[DEPLOYED]\x1b[0m website deployed using vercel: {}", repo_folder);

                let deployment_url = format!("https://{}.vercel.app", repo_name);
                if let Err(err) = save_deployment_info_to_md(file, &repo_name, &deployment_url) {
                    println!("\x1b[31m[ERROR]\x1b[0m {}", err);
                }
            },
            Ok(status) => println!("\x1b[31m[ERROR]\x1b[0m failed to deploy using vercel. Exit code: {}", status),
            Err(err) => println!("\x1b[31m[ERROR]\x1b[0m failed to run 'vercel' command: {}", err),
        }
    }
}

fn save_deployment_info_to_md(file: &mut File, repo_name: &str, deployment_url: &str) -> Result<()> {
    writeln!(file, "[{}]({})\n", repo_name, deployment_url)?;
    Ok(())
}
