use colored::Colorize;
use prettytable::{format, row, Row, Table};

use crate::tasks::{Task, Tasks};

pub fn calc_row(task: &Task, id: usize) -> Row {
    if task.is_complete() {
        // Generate greyed out rows for complete tasks
        Row::from([
            id.to_string().bright_black().italic(),
            task.status_string().bright_black().italic(),
            task.title_string().bright_black().italic(),
            task.when_string().bright_black().italic(),
            task.deadline_string().bright_black().italic(),
        ])
    } else {
        // Generate normal colored rows for uncompleted tasks
        Row::from([
            id.to_string().cyan(),
            task.status_string(),
            task.title_string(),
            task.when_string(),
            task.deadline_string(),
        ])
    }
}

pub fn tasks_table(tasks: &Tasks) -> Table {
    // Create the table for printing
    let mut table = Table::new();
    table.set_titles(row![
        "ID".magenta().bold(),
        "Status".magenta().bold(),
        "Title".magenta().bold(),
        "When".magenta().bold(),
        "Deadline".magenta().bold(),
    ]);
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    // Iterate through each task
    for (id, task) in tasks.tasks.as_ref().unwrap().iter().enumerate() {
        table.add_row(calc_row(task, id));
    }

    table
}

pub fn task_table(task: &Task, id: usize) -> Table {
    let mut table = Table::new();
    table.set_titles(row!["Item".magenta().bold(), "Value".magenta().bold()]);
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    // Add rows
    table.add_row(row!["ID".white().bold(), id.to_string().cyan()]);
    table.add_row(row!["Status".white().bold(), task.status_string()]);
    table.add_row(row!["Title".white().bold(), task.title_string()]);
    table.add_row(row!["When".white().bold(), task.when_string(),]);
    table.add_row(row!["Deadline".white().bold(), task.deadline_string(),]);
    table.add_row(row!["Reminder".white().bold(), task.reminder_string(),]);
    table.add_row(row!["Tags".white().bold(), &task.tags_string()]);
    table.add_row(row!["Notes".white().bold(), &task.notes_string()]);

    table
}
