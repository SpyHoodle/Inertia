use dirs::home_dir;
use std::error::Error;
use std::fs;
use std::path::Path;

use crate::tasks::Tasks;

const TASKS_FILE_PATH: &str = "/.local/share/tasks";

pub fn save_tasks<P: AsRef<Path>>(path: P, tasks: &Tasks) -> Result<(), Box<dyn Error>> {
    // Convert the tasks to TOML format
    let data = toml::to_string_pretty(&tasks)?;

    // Write the JSON to the file
    fs::write(path, data)?;

    Ok(())
}

pub fn load_tasks<P: AsRef<Path>>(path: P) -> Result<Tasks, Box<dyn Error>> {
    // Read JSON from the file
    let data = fs::read_to_string(path)?;

    // Load the tasks from TOML form
    let tasks: Tasks = toml::from_str(&data)?;

    Ok(tasks)
}

pub fn tasks_file_path() -> String {
    // Generate the path for the location of tasks
    format!(
        "{}{}",
        home_dir().unwrap().to_str().unwrap(),
        TASKS_FILE_PATH
    )
}
