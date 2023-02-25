use colored::Colorize;
use prettytable::{format, row, Row, Table};

use crate::cli::dates;
use crate::tasks::{Status, Task, Tasks};

pub fn calc_row(task: &Task, id: usize) -> Row {
    if task.status == Status::Complete {
        // Generate greyed out rows for complete tasks
        Row::from([
            id.to_string().bright_black().italic(),
            task.status.as_string().bright_black().italic(),
            task.title.clone().bright_black().italic(),
            dates::date_as_string(&task.when).bright_black().italic(),
            dates::date_as_string(&task.deadline)
                .bright_black()
                .italic(),
        ])
    } else {
        // Generate normal colored rows for uncompleted tasks
        Row::from([
            id.to_string().cyan(),
            task.status.as_string(),
            task.title.clone().white(),
            dates::date_as_string(&task.when),
            dates::date_as_string(&task.deadline),
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
    table.add_row(calc_row(task, id));

    table
}
