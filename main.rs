use std::process::{Command};
use std::fs;

const WEBSITE_REPO_URLS: [&str; 2] = [
    "https://github.com/ivoinestrachan/haunted-house-testing",
    "https://github.com/ShubhamPatilsd/wandering-wavelength"
];
const TARGET_FOLDER: &str = "www";

fn main() {
    clone_website_repositories();
}

fn clone_website_repositories() {
    for repo_url in WEBSITE_REPO_URLS.iter() {
        let repo_name = repo_url.split('/').last().unwrap_or("repo");
        let repo_folder = format!("{}/{}", TARGET_FOLDER, repo_name);

        if fs::metadata(&repo_folder).is_ok() {
            println!("repository {} already exists, skipping.....", repo_folder);
            continue;
        }

        println!("cloning repository from {} into folder {}", repo_url, repo_folder);

        let result = Command::new("git")
            .arg("clone")
            .arg(repo_url)
            .arg(&repo_folder)
            .output();

        match result {
            Ok(_output) => println!("git cloned {}", repo_folder),
            Err(err) => println!("failed to clone repository: {}", err),
        }

        if let Err(err) = fs::create_dir_all(&repo_folder) {
            println!("failed to create directory: {}", err);
            continue;
        }

        if let Err(err) = Command::new("sh")
            .arg("-c")
            .arg(format!("cd {} && vercel --yes", repo_folder))
            .status()
        {
            println!("failed to run 'vercel' command: {}", err);
        }
    }
}
