mod cmds;
mod dates;
pub mod git;
pub mod output;
mod tables;

use crate::args::{Commands, GitExecute, TasksArgs};
use crate::args::{
    CompleteTask, CreateTask, DeleteTask, ModifyTask, ShowTask, StartTask, StopTask, SyncTasks,
};
use crate::tasks::{Tasks, TasksError};

pub fn execute(tasks: &mut Tasks, arguments: TasksArgs) -> Result<(), TasksError> {
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

        Commands::Git(GitExecute { command }) => match git::execute(&tasks.path, command) {
            Ok(..) => (),
            Err(..) => panic!("failed to execute git cmd"),
        },

        Commands::Sync(SyncTasks { remote }) => match git::sync(&tasks.path, remote) {
            Ok(..) => (),
            Err(..) => panic!("failed"),
        },

        _ => todo!(),
    };
    Ok(())
}
