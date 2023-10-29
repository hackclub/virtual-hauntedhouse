use rayon::prelude::*;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Result, Write};
use std::process::{Command, Stdio};

const WEBSITE_REPO_URLS: &[&str] = &[
    "https://github.com/anthonyk2923/haunted-house-hackathon",
    "https://github.com/CaptainBlowFish/spookyscarrysite",
    "https://github.com/esjiko/miffyshouse",
    "https://github.com/edcous/haunted-house",
    "https://github.com/hamiltrashh/fivenightsinfrance",
    "https://github.com/MidnightPaws/haunted-mansion-room",
    "https://github.com/LJthegreat1/three-keys",
    "https://github.com/ymsolis/3doors"


];
const TARGET_FOLDER: &str = "www";

fn main() {
    let mut file = match OpenOptions::new()
        .append(true)
        .create(true)
        .open("links.md")
    {
        Ok(f) => f,
        Err(e) => {
            println!("\x1b[31m[ERROR]\x1b[0m failed to open links.md file: {}", e);
            return;
        }
    };

    clone_and_deploy_websites(&mut file);
}

fn clone_and_deploy_websites(file: &mut File) {
    struct Res {
        repo_name: String,
        deployment_url: String,
    }

    let deployment_results: Vec<Res> = WEBSITE_REPO_URLS
        .par_iter()
        .filter_map(|repo_url| {
            let repo_name = repo_url.split('/').last().unwrap_or("repo");
            let repo_folder = format!("{}/{}", TARGET_FOLDER, repo_name);

            if fs::metadata(&repo_folder).is_ok() {
                println!(
                    "\x1b[33m[SKIPPING]\x1b[0m repository {} already exists. skipping clone.",
                    repo_folder
                );
            } else {
                println!(
                    "\x1b[36m[CLONING]\x1b[0m cloning repository from {} into folder {}",
                    repo_url, repo_folder
                );

                let git_clone_result = Command::new("git")
                    .arg("clone")
                    .arg(repo_url)
                    .arg(&repo_folder)
                    .output();

                match git_clone_result {
                    Ok(_output) => println!(
                        "\x1b[32m[SUCCESS]\x1b[0m repository {} cloned successfully.",
                        repo_folder
                    ),
                    Err(err) => {
                        println!("\x1b[31m[ERROR]\x1b[0m failed to clone repository: {}", err)
                    }
                }

                if let Err(err) = fs::create_dir_all(&repo_folder) {
                    println!("\x1b[31m[ERROR]\x1b[0m failed to create directory: {}", err);
                    return None;
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
                        println!(
                            "\x1b[32m[DEPLOYED]\x1b[0m website deployed using vercel: {}",
                            repo_folder
                        );

                        let deployment_url = format!("https://{}.vercel.app", repo_name);
                        return Some(Res {
                            repo_name: repo_name.to_owned(),
                            deployment_url,
                        });
                    }
                    Ok(status) => println!(
                        "\x1b[31m[ERROR]\x1b[0m failed to deploy using vercel. Exit code: {}",
                        status
                    ),
                    Err(err) => println!(
                        "\x1b[31m[ERROR]\x1b[0m failed to run 'vercel' command: {}",
                        err
                    ),
                }
            }
            None
        })
        .collect::<Vec<_>>();

    for Res {
        deployment_url,
        repo_name,
    } in deployment_results
    {
        if let Err(err) = save_deployment_info_to_md(file, &repo_name, &deployment_url) {
            println!("\x1b[31m[ERROR]\x1b[0m {}", err);
        }
    }
}

fn save_deployment_info_to_md(
    file: &mut File,
    repo_name: &str,
    deployment_url: &str,
) -> Result<()> {
    writeln!(file, "[{}]({})\n", repo_name, deployment_url)?;
    Ok(())
}
