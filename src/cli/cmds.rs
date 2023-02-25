use crate::cli::dates;
use crate::cli::output;
use crate::cli::tables;
use crate::tasks::{Task, Tasks, TasksError};

fn parse_tags(tags: Option<String>) -> Option<Vec<String>> {
    if let Some(..) = tags {
        Some(tags.unwrap().split(',').map(str::to_string).collect())
    } else {
        None
    }
}

pub fn show(tasks: &mut Tasks, id: Option<usize>) -> Result<(), TasksError> {
    // If no id is given, print out all tasks
    if let Some(..) = id {
        if tasks.is_empty() {
            // Output when no tasks are available
            output::info(String::from("no tasks found"))
        } else {
            // Generate the table of all tasks
            let table = tables::tasks_table(tasks);
            // Print the table
            println!("{}", table);
        };
    } else {
        // Get the task the user wants to see
        let id = id.unwrap();
        let task = tasks.get_task(id)?;

        // Generate the table for the singular task
        let table = tables::task_table(task, id);
        // Print the table
        println!("{}", table);
    };

    // Success
    Ok(())
}

pub fn add(
    tasks: &mut Tasks,
    title: String,
    notes: Option<String>,
    tags: Option<String>,
    when: Option<String>,
    deadline: Option<String>,
    reminder: Option<String>,
) {
    // Parse dates and tags
    let when = dates::parse_fuzzy_date(when);
    let deadline = dates::parse_fuzzy_date(deadline);
    let reminder = dates::parse_fuzzy_date(reminder);
    let tags = parse_tags(tags);

    // Generate a new task
    let task = Task::new(title, notes, tags, when, deadline, reminder);
    // Add the task to the tasks
    tasks.push(task.clone());

    // Calculate the id for output
    let id = tasks.len() - 1;

    // Success
    output::success(output::task_msg("created", &task, id));
}

pub fn modify(
    tasks: &mut Tasks,
    id: usize,
    title: Option<String>,
    notes: Option<String>,
    tags: Option<String>,
    when: Option<String>,
    deadline: Option<String>,
    reminder: Option<String>,
) -> Result<(), TasksError> {
    // Parse dates and tags
    let when = dates::parse_fuzzy_date(when);
    let deadline = dates::parse_fuzzy_date(deadline);
    let reminder = dates::parse_fuzzy_date(reminder);
    let tags = parse_tags(tags);

    // Get the task the user wants
    let task = tasks.get_task(id)?;

    // If the the user changes the title, show that here
    if title.is_some() {
        output::info(output::task_msg("renaming task", task, id))
    };

    // Modify the task
    task.modify(title, notes, tags, when, deadline, reminder);

    // Success
    output::success(output::task_msg("modified", task, id));
    Ok(())
}

pub fn delete(tasks: &mut Tasks, id: usize) -> Result<(), TasksError> {
    // Get the task the user wants to delete for output later
    let mut binding = tasks.clone();
    let task = binding.get_task(id)?;

    // Delete the task
    tasks.remove(id)?;

    // Success
    output::success(output::task_msg("deleted", task, id));
    Ok(())
}

pub fn clear(tasks: &mut Tasks) -> Result<(), TasksError> {
    // Clear all tasks
    tasks.clear()?;

    // Success
    output::success(String::from("cleared all tasks"));
    Ok(())
}

pub fn stop(tasks: &mut Tasks, id: usize) -> Result<(), TasksError> {
    // Get the task the user wants to stop
    let task = tasks.get_task(id)?;
    // Stop the task
    task.stop();

    // Success
    output::success(output::task_msg("stopped", task, id));
    Ok(())
}

pub fn start(tasks: &mut Tasks, id: usize) -> Result<(), TasksError> {
    // Get the task the user wants to start
    let task = tasks.get_task(id)?;
    // Start the task
    task.start();

    // Success
    output::success(output::task_msg("started", task, id));
    Ok(())
}

pub fn done(tasks: &mut Tasks, id: usize) -> Result<(), TasksError> {
    // Get the task the user wants to complete
    let task = tasks.get_task(id)?;
    // Complete the task
    task.complete();

    // Success
    output::success(output::task_msg("completed", task, id));
    Ok(())
}
