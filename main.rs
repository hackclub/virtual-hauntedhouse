use std::process::Command;

const WEBSITE_REPO_URLS: [&str; 2] = [
    "https://github.com/ivoinestrachan/haunted-house-testing.git",
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

        println!("cloning repository from {} into folder {}", repo_url, repo_folder);

        let result = Command::new("git")
            .arg("clone")
            .arg(repo_url)
            .arg(&repo_folder)
            .output();

        match result {
            Ok(output) => println!("git cloned: {}", String::from_utf8_lossy(&output.stdout)),
            Err(err) => println!("failed to clone repository: {}", err),
        }
    }
}
