use crate::tasks::Task;
use colored::Colorize;

pub fn warning(msg: &str) {
    println!("{} {}", "warning:".yellow().bold(), msg);
}

pub fn info(msg: String) {
    println!("{} {}", "info:".blue().bold(), msg);
}

pub fn success(msg: String) {
    println!("{} {}", "success:".green().bold(), msg);
}

pub fn task_msg(msg: &str, task: &Task, id: usize) -> String {
    format!(
        "{} task: {}({})",
        msg,
        task.title.blue(),
        id.to_string().cyan()
    )
}
