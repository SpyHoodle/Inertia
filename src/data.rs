use dirs::home_dir;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::string::ToString;

use crate::cli::git;
use crate::cli::output;
use crate::tasks::Tasks;

pub fn save_tasks<P: AsRef<Path>>(path: P, tasks: &Tasks) -> Result<(), Box<dyn Error>> {
    // Convert the tasks to TOML format
    let data = toml::to_string_pretty(&tasks)?;

    // Write the TOML to the file
    fs::write(path, data)?;

    Ok(())
}

pub fn load_tasks<P: AsRef<Path> + ToString>(
    path: P,
    tasks_file: &str,
) -> Result<Tasks, Box<dyn Error>> {
    let tasks_file_path = &format!("{}/{}", path.to_string(), tasks_file);

    // Read TOML from the file
    let data = fs::read_to_string(tasks_file_path)?;

    // Load the tasks from TOML form
    let tasks: Tasks = toml::from_str(&data)?;

    Ok(tasks)
}

pub fn ensure_repo(path: &str, tasks_file: &str) -> Result<(), Box<dyn Error>> {
    // Generate the path of the tasks file
    let tasks_file_path = &format!("{}/{}", path, tasks_file);

    // Check if the path exists
    if !Path::new(path).exists() {
        output::warning(format!(
            "tasks repository {path} does not exist. creating..."
        ));
        fs::create_dir_all(path).unwrap();
        let tasks = Tasks::new(path, tasks_file);
        save_tasks(tasks_file_path, &tasks).unwrap();
        git::execute(path, String::from("init"))?;
        git::execute(path, String::from("add ."))?;
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
