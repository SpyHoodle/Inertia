mod args;
mod cli;
mod repo;
mod tasks;

use clap::Parser;
use colored::*;

use crate::args::TasksArgs;

fn main() {
    // Generate the file paths for tasks
    let repo_path = repo::tasks_repo_string();
    let tasks_file_path = repo::tasks_file_path();

    // If the tasks file doesn't exist, create it first
    match repo::ensure_repo(&repo_path) {
        Ok(..) => (),
        Err(error) => panic!("{} {:?}", "error:".red().bold(), error),
    };

    // Load tasks and check for any errors when loading the tasks
    let mut tasks = match repo::load_tasks(&tasks_file_path) {
        Ok(tasks) => tasks,
        Err(error) => panic!("{} {:?}", "error:".red().bold(), error),
    };

    // Parse command line arguments
    let arguments = TasksArgs::parse();

    // Execute the inputted command line arguments
    match cli::execute(&mut tasks, arguments) {
        Ok(..) => (),
        Err(error) => panic!("{} {:?}", "error:".red().bold(), error),
    };

    // Save any changes
    repo::save_tasks(&tasks_file_path, &tasks).unwrap();
    repo::execute(&repo_path, String::from("add --all")).unwrap();
}
