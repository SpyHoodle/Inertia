use dirs::home_dir;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::string::ToString;

use crate::cli::output;
use crate::tasks::Tasks;

const TASKS_FILE: &str = "tasks.toml";

pub fn execute(path: &str, command: String) -> Result<(), Box<dyn Error>> {
    let output = Command::new("git")
        .args(["-C", path])
        .args(command.split(' '))
        .output()?;

    if !output.stdout.is_empty() {
        output::git(String::from_utf8(output.stdout).unwrap());
    };
    if !output.stderr.is_empty() {
        output::error(String::from_utf8(output.stderr).unwrap());
    };

    Ok(())
}

pub fn save_tasks<P: AsRef<Path>>(path: P, tasks: &Tasks) -> Result<(), Box<dyn Error>> {
    // Convert the tasks to TOML format
    let data = toml::to_string_pretty(&tasks)?;

    // Write the TOML to the file
    fs::write(path, data)?;

    Ok(())
}

pub fn load_tasks<P: AsRef<Path> + ToString>(path: P) -> Result<Tasks, Box<dyn Error>> {
    // Read TOML from the file
    let data = fs::read_to_string(path)?;

    // Load the tasks from TOML form
    let tasks: Tasks = toml::from_str(&data)?;

    Ok(tasks)
}

pub fn ensure_repo(path: &str) -> Result<(), Box<dyn Error>> {
    // Generate the path of the tasks file
    let tasks_file_path = tasks_file_path();

    // Check if the path exists
    if !Path::new(path).exists() {
        output::warning(format!(
            "tasks repository {path} does not exist. creating..."
        ));

        // Create the directory
        fs::create_dir_all(path).unwrap();
        // Generate a new empty tasks structure
        let tasks = Tasks::new(path, TASKS_FILE);

        // Save the tasks
        save_tasks(tasks_file_path, &tasks).unwrap();

        // Create the git repository
        execute(path, String::from("init"))?;
        execute(path, String::from("add --all"))?;

        // Success
        output::success(format!("created tasks repo {path}"));
    }

    Ok(())
}

pub fn tasks_repo_string() -> String {
    // Generate the path for the location of tasks
    let home_dir = home_dir().unwrap();
    let home_dir = home_dir.to_str().unwrap();
    format!("{home_dir}/.local/share/inertia")
}

pub fn tasks_file_path() -> String {
    format!("{}/{}", tasks_repo_string(), TASKS_FILE)
}

pub fn sync(repo_path: &str, remote: String) -> Result<(), Box<dyn Error>> {
    execute(
        repo_path,
        format!("pull --ff --no-rebase --no-edit --commit {remote}"),
    )?;
    execute(repo_path, format!("push {remote}"))?;

    Ok(())
}
