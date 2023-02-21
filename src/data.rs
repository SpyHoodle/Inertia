use std::error::Error;
use std::fs;
use std::path::Path;
use dirs::home_dir;
use serde_json;

use crate::tasks::Tasks;


const TASKS_FILE_PATH: &str = "/.local/share/tasks";

pub fn save_tasks<P: AsRef<Path>>(path: P, tasks: &Tasks) -> Result<(), Box<dyn Error>> {
    // Convert the tasks to JSON form
    let data = serde_json::to_string_pretty(&tasks)?;

    // Write the JSON to the file
    fs::write(path, data)?;

    Ok(())
}

pub fn load_tasks<P: AsRef<Path>>(path: P) -> Result<Tasks, Box<dyn Error>> {
    // Read JSON from the file
    let data = fs::read_to_string(path)?;

    // Load the tasks from JSON form
    let tasks: Tasks = serde_json::from_str(&data)?;

    Ok(tasks)
}

pub fn tasks_file_path() -> String {
    format!("{}{}", home_dir().unwrap().to_str().unwrap(), TASKS_FILE_PATH)
}
