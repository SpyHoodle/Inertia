use crate::tasks::Task;
use colored::Colorize;

pub fn error(msg: String) {
    println!("{} {}", "error".red().bold(), msg);
}

pub fn warning(msg: String) {
    println!("{} {}", "warning:".yellow().bold(), msg);
}

pub fn info(msg: String) {
    println!("{} {}", "info:".blue().bold(), msg);
}

pub fn git(msg: String) {
    let msg = msg.strip_suffix('\n').unwrap_or(&msg);
    println!("{} {}", "git:".blue().bold(), msg.bright_black().italic());
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
