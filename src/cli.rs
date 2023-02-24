use chrono::{NaiveDateTime, Local};
use crate::args::{TasksArgs, Commands};
use crate::args::{CreateTask, DeleteTask, ShowTask, StartTask, StopTask, CompleteTask};
use crate::tasks::{Tasks, Task, Status, TasksError};
use prettytable::{Table, Row, row, format};
use colored::*;
use fuzzydate;
use std::panic;


fn success(msg: String) {
    println!("{} {}", "success:".green().bold(), msg);
}

pub fn warning(msg: &str) {
    println!("{} {}", "warning:".yellow().bold(), msg);
}

pub fn info(msg: &str) {
    println!("{} {}", "info:".blue().bold(), msg);
}

fn task_msg(msg: &str, task: &Task, id: usize) -> String {
    format!("{} task: {}({})", msg, task.title.blue(), id.to_string().cyan())
}

fn parse_date(date_string: Option<String>) -> Option<NaiveDateTime> {
    if date_string.is_some() {
        match fuzzydate::parse(date_string.unwrap()) {
            Ok(date) => Some(date),
            Err(err) => panic!("{:?}", err),
        }
    } else {
        None
    }
}

fn calc_row(task: &Task, id: usize) -> Row {
    if task.status == Status::Complete {
        // Generate greyed out rows for complete tasks
        Row::from([id.to_string().bright_black().italic(),
            task.status.as_string().bright_black().italic(),
            "N/A".bright_black(),
            task.title.clone().bright_black().italic()])

    } else {
        let when = if task.when.is_some() {
            let date = format!("{}", task.when.unwrap().format("%Y-%m-%d"));
            let now = Local::now().date_naive();

            if now == task.when.unwrap().date() {
                date.bright_red()
            } else if now.succ_opt().unwrap() == task.when.unwrap().date() {
                date.yellow()
            } else {
                date.white()
            }
        } else {
            "N/A".bright_black()
        };

        // Generate normal colored rows for uncompleted tasks
        Row::from([id.to_string().cyan(), task.status.as_string(), when, task.title.clone().white()])
    }
}

pub fn execute(tasks: &mut Tasks, arguments: TasksArgs) -> Result<&mut Tasks, TasksError> {
    match arguments.command {
        Commands::Add(CreateTask { title, notes, tags, when, deadline, reminder, ..}) => {
            let when = parse_date(when);
            let deadline = parse_date(deadline);
            let reminder = parse_date(reminder);
            let tags: Option<Vec<String>> = if tags.is_some() {
                Some(tags.unwrap().split(",").map(str::to_string).collect())
            } else {
                None
            };

            let task = Task::new(title, notes, tags, when, deadline, reminder);
            tasks.push(task.clone());

            let id = tasks.len() - 1;
            success(task_msg("created", &task, id));

            Ok(tasks)
        }

        Commands::Del(DeleteTask { id }) => {
            let mut binding = tasks.clone();
            let task = binding.get_task(id)?;
            tasks.remove(id)?;

            success(task_msg("deleted", &task, id));
            Ok(tasks)
        }

        Commands::Done(CompleteTask { id }) => {
            let task = tasks.get_task(id)?;
            task.complete();

            success(task_msg("completed", &task, id));
            Ok(tasks)
        }

        Commands::Start(StartTask { id }) => {
            let task = tasks.get_task(id)?;
            task.start();

            success(task_msg("started", &task, id));
            Ok(tasks)
        }

        Commands::Stop(StopTask { id }) => {
            let task = tasks.get_task(id)?;
            task.stop();

            success(task_msg("stopped", &task, id));
            Ok(tasks)
        }

        Commands::Clear => {
            tasks.clear()?;

            success(String::from("cleared all tasks"));
            Ok(tasks)
        }

        Commands::Show(ShowTask { id }) => {
            if id.is_none() {
                if tasks.is_empty() {
                    info("no tasks found")
                } else {
                    // Create the table for printing
                    let mut table = Table::new();
                    table.set_titles(row!["ID".magenta().bold(), "Status".magenta().bold(), "When".magenta().bold(), "Title".magenta().bold()]);
                    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

                    // Iterate through each task
                    let mut id = 0;
                    for task in tasks.tasks.as_ref().unwrap() {
                        table.add_row(calc_row(task, id));
                        id += 1;
                    }

                    // Print the table
                    println!("{}", table);
                };
            } else {
                // Get the task
                let id = id.unwrap();
                let task = tasks.get_task(id)?;

                // Generate and print the table
                let mut table = Table::new();
                table.set_titles(row!["ID".magenta().bold(), "Status".magenta().bold(), "When".magenta().bold(), "Title".magenta().bold()]);
                table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
                table.add_row(calc_row(&task, id));
                println!("{}", table)
            };

            Ok(tasks)
        }

        _ => todo!()
    }
}
