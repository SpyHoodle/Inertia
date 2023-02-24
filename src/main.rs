mod tasks;
mod args;
mod data;
mod cli;

use crate::tasks::Tasks;
use crate::args::TasksArgs;
use std::path::Path;
use clap::Parser;
use colored::*;

fn main() {
    // Generate the file path for tasks
    let tasks_file_path = data::tasks_file_path();

    // If the tasks file doesn't exist, create it first
    if !Path::new(&tasks_file_path).exists() {
        cli::warning("file '~/.local/share/tasks' does not exist. creating..");
        let tasks = Tasks::new(&tasks_file_path);
        data::save_tasks(&tasks_file_path, &tasks).unwrap();
    };

    // Load tasks and check for any errors when loading the tasks
    let mut tasks = match data::load_tasks(&tasks_file_path) {
        Ok(tasks) => tasks,
        Err(error) => panic!("{} {:?}", "error:".red().bold(), error),
    };

    // Parse command line arguments
    let arguments = TasksArgs::parse();
    let tasks = match cli::execute(&mut tasks, arguments) {
        Ok(tasks) => tasks,
        Err(error) => panic!("{} {:?}", "error:".red().bold(), error),
    };

    // Save any changes
    data::save_tasks(tasks_file_path, &tasks).unwrap()
}
