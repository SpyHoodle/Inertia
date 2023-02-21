use crate::args::{TasksArgs, Commands};
use crate::args::{CreateTask, DeleteTask, ShowTask, StartTask, StopTask, CompleteTask};
use crate::tasks::{Tasks, Task, Status};
use prettytable::{Table, Row, row, format};
use colored::*;

fn success(msg: String) {
    println!("{} {}", "success:".green().bold(), msg);
}

fn warning(msg: &str) {
    println!("{} {}", "warning:".yellow().bold(), msg);
}

#[allow(dead_code)]
fn error(msg: String) {
    println!("{} {}", "error:".red().bold(), msg);
    panic!();
}

fn task_msg(msg: &str, task: &Task, id: usize) -> String {
    format!("{} task: {}({})", msg, task.title.blue(), id.to_string().cyan())
}

fn get_task(tasks: &mut Tasks, id: usize) -> Task {
    match tasks.get_task(id) {
        Ok(task) => task.clone(),
        Err(error) => panic!("error: {}", error),
    }
}

fn calc_row(task: &Task, id: usize) -> Row {
    if task.status == Status::Complete {
        // Generate greyed out rows for complete tasks
        Row::from([id.to_string().bright_black().italic(),
            task.status.as_string().bright_black().italic(),
            task.title.clone().bright_black().italic()])
    } else {
        // Generate normal colored rows for uncompleted tasks
        Row::from([id.to_string().cyan(), task.status.as_string(), task.title.clone().white()])
    }
}

pub fn execute(tasks: &mut Tasks, arguments: TasksArgs) -> &mut Tasks {
    match arguments.command {
        Commands::Add(CreateTask { title, .. }) => {
            let task = Task::new(title);
            tasks.add(task.clone());

            let id = tasks.len() - 1;
            success(task_msg("created", &task, id));
        }
        Commands::Del(DeleteTask { id }) => {
            let mut binding = tasks.clone();
            let task = match binding.get_task(id) {
                Ok(task) => task,
                Err(error) => panic!("error: {}", error),
            };

            tasks.del(id);
            success(task_msg("deleted", task, id));
        }
        Commands::Show(ShowTask { id }) => {
            if id.is_none() {
                if tasks.tasks.is_none() {
                    warning("no tasks available to show")
                } else {
                    // Create the table for printing
                    let mut table = Table::new();
                    table.set_titles(row!["ID".magenta().bold(), "Status".magenta().bold(), "Title".magenta().bold()]);
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
                let id = id.unwrap();
                let task = get_task(&mut tasks.clone(), id);

                // Generate and print the table
                let mut table = Table::new();
                table.set_titles(row!["ID".magenta().bold(), "Status".magenta().bold(), "Title".magenta().bold()]);
                table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
                table.add_row(calc_row(&task, id));
                println!("{}", table)
            };
        }
        Commands::Done(CompleteTask { id }) => {
            let task = get_task(&mut tasks.clone(), id);

            tasks.set_status(id, Status::Complete);
            success(task_msg("completed", &task, id));
        }
        Commands::Start(StartTask { id }) => {
            let task = get_task(&mut tasks.clone(), id);

            tasks.set_status(id, Status::Active);
            success(task_msg("started", &task, id));
        }
        Commands::Stop(StopTask { id }) => {
            let task = get_task(&mut tasks.clone(), id);

            if task.when.is_none() {
                tasks.set_status(id, Status::Inbox);
            } else {
                tasks.set_status(id, Status::Pending);
            };
            success(task_msg("stopped", &task, id));
        }
        Commands::Clear => {
            tasks.clear();
            success(String::from("cleared all tasks"));
        }
        _ => todo!()
    };
    tasks
}