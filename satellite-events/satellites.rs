use std::process::{Command, Stdio};
use std::fs;
use std::fs::File;
use std::io::{Write, Result};
use std::fs::OpenOptions;

struct Location {
    name: &'static str,
    repo_urls: &'static [&'static str],
}

const LOCATIONS: [Location; 2] = [
    Location {
        name: "bayarea",
        repo_urls: &[
            "https://github.com/ivoinestrachan/haunted-house-testing",
            "https://github.com/ShubhamPatilsd/wandering-wavelength",
        ],
    },
    Location {
        name: "toronto",
        repo_urls: &[
            "https://github.com/ivoinestrachan/haunted-house-testing",
            "https://github.com/ShubhamPatilsd/wandering-wavelength",
        ],
    },
];

fn main() {
    for location in LOCATIONS.iter() {
        let mut location_file = match OpenOptions::new().append(true).create(true).open(format!("{}.md", location.name)) {
            Ok(f) => f,
            Err(e) => {
                println!("\x1b[31m[ERROR]\x1b[0m failed to open {}.md file: {}", location.name, e);
                return;
            }
        };

        clone_and_deploy_projects(&mut location_file, location);
    }
}

fn clone_and_deploy_projects(file: &mut File, location: &Location) {
    for repo_url in location.repo_urls.iter() {
        let repo_name = repo_url.split('/').last().unwrap_or("repo");
        let project_folder = format!("{}/{}", location.name, repo_name);

        if fs::metadata(&project_folder).is_ok() {
            println!("\x1b[33m[SKIPPING]\x1b[0m project {} in {} already exists. Skipping clone.", repo_name, location.name);
        } else {
            println!("\x1b[36m[CLONING]\x1b[0m cloning repository from {} into folder {}/{}", repo_url, location.name, repo_name);

            let git_clone_result = Command::new("git")
                .arg("clone")
                .arg(repo_url)
                .arg(&project_folder)
                .output();

            match git_clone_result {
                Ok(_output) => println!("\x1b[32m[SUCCESS]\x1b[0m repository {} cloned successfully.", project_folder),
                Err(err) => println!("\x1b[31m[ERROR]\x1b[0m failed to clone repository: {}", err),
            }

            let vercel_command = format!("cd {} && vercel --yes", project_folder);
            let shell_result = Command::new("sh")
                .arg("-c")
                .arg(&vercel_command)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status();

            match shell_result {
                Ok(status) if status.success() => {
                    println!("\x1b[32m[DEPLOYED]\x1b[0m website deployed using vercel: {}", project_folder);

                    let deployment_url = format!("https://{}.vercel.app", repo_name);
                    if let Err(err) = save_deployment_info_to_md(file, &project_folder, &deployment_url) {
                        println!("\x1b[31m[ERROR]\x1b[0m {}", err);
                    }
                },
                Ok(status) => println!("\x1b[31m[ERROR]\x1b[0m failed to deploy using vercel. exit code: {}", status),
                Err(err) => println!("\x1b[31m[ERROR]\x1b[0m failed to run 'vercel' command: {}", err),
            }
        }
    }
}

fn save_deployment_info_to_md(file: &mut File, project_folder: &str, deployment_url: &str) -> Result<()> {
    writeln!(file, "[{}]({})\n", project_folder, deployment_url)?;
    Ok(())
}
