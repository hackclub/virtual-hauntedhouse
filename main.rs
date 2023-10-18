use std::process::{Command, Stdio};
use std::fs;

const WEBSITE_REPO_URLS: [&str; 2] = [
    "https://github.com/ivoinestrachan/haunted-house-testing",
    "https://github.com/ShubhamPatilsd/wandering-wavelength"
];
const TARGET_FOLDER: &str = "www";

fn main() {
    clone_and_deploy_websites();
}

fn clone_and_deploy_websites() {
    for repo_url in WEBSITE_REPO_URLS.iter() {
        let repo_name = repo_url.split('/').last().unwrap_or("repo");
        let repo_folder = format!("{}/{}", TARGET_FOLDER, repo_name);

        if fs::metadata(&repo_folder).is_ok() {
            println!("\x1b[33m[SKIPPING]\x1b[0m repository {} already exists. skipping clone.", repo_folder);
            continue;
        }

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
            continue;
        }

        let vercel_command = format!("cd {} && vercel --yes", repo_folder);
        let shell_result = Command::new("sh")
            .arg("-c")
            .arg(&vercel_command)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status();

        match shell_result {
            Ok(status) if status.success() => println!("\x1b[32m[DEPLOYED]\x1b[0m website deployed using vercel: {}", repo_folder),
            Ok(status) => println!("\x1b[31m[ERROR]\x1b[0m failed to deploy using vercel. Exit code: {}", status),
            Err(err) => println!("\x1b[31m[ERROR]\x1b[0m failed to run 'vercel' command: {}", err),
        }
    }
}
