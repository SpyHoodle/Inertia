mod dates;
pub mod output;
mod tables;

mod cmds;

use crate::args::{Commands, TasksArgs};
use crate::args::{
    CompleteTask, CreateTask, DeleteTask, ModifyTask, ShowTask, StartTask, StopTask,
};
use crate::tasks::{Tasks, TasksError};

pub fn execute(tasks: &mut Tasks, arguments: TasksArgs) -> Result<&mut Tasks, TasksError> {
    match arguments.command {
        Commands::Add(CreateTask {
            title,
            notes,
            tags,
            when,
            deadline,
            reminder,
        }) => {
            cmds::add(tasks, title, notes, tags, when, deadline, reminder);
        }

        Commands::Modify(ModifyTask {
            id,
            title,
            notes,
            tags,
            when,
            deadline,
            reminder,
        }) => {
            cmds::modify(tasks, id, title, notes, tags, when, deadline, reminder)?;
        }

        Commands::Del(DeleteTask { id }) => {
            cmds::delete(tasks, id)?;
        }

        Commands::Done(CompleteTask { id }) => {
            cmds::done(tasks, id)?;
        }

        Commands::Start(StartTask { id }) => {
            cmds::start(tasks, id)?;
        }

        Commands::Stop(StopTask { id }) => {
            cmds::stop(tasks, id)?;
        }

        Commands::Clear => {
            cmds::clear(tasks)?;
        }

        Commands::Show(ShowTask { id }) => {
            cmds::show(tasks, id)?;
        }

        _ => todo!(),
    };

    Ok(tasks)
}
