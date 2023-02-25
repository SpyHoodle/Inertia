mod args;
mod cli;
mod data;
mod tasks;

use clap::Parser;
use colored::*;

use crate::args::TasksArgs;

fn main() {
    // Generate the file paths for tasks
    let repo_path = &data::tasks_repo_string();
    let tasks_file = "tasks";

    // If the tasks file doesn't exist, create it first
    match data::ensure_repo(repo_path, tasks_file) {
        Ok(..) => (),
        Err(error) => panic!("{} {:?}", "error:".red().bold(), error),
    };

    // Load tasks and check for any errors when loading the tasks
    let mut tasks = match data::load_tasks(repo_path, tasks_file) {
        Ok(tasks) => tasks,
        Err(error) => panic!("{} {:?}", "error:".red().bold(), error),
    };

    // Parse command line arguments
    let arguments = TasksArgs::parse();
    match cli::execute(&mut tasks, arguments) {
        Ok(..) => (),
        Err(error) => panic!("{} {:?}", "error:".red().bold(), error),
    };

    // Save any changes
    cli::git::execute(repo_path, String::from("add --all")).unwrap();
    data::save_tasks(&repo_path, &tasks).unwrap();
}
