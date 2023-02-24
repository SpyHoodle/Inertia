use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use colored::*;


#[derive(Debug)]
pub struct TasksError(String);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Status {
    Inbox,
    Pending,
    Active,
    Complete,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub title: String,               // The required title of the task
    pub status: Status,              // Current status of the task
    pub notes: Option<String>,           // Any notes to explain the task
    pub tags: Option<Vec<String>>,          // Tasks can be tagged for organisation
    pub subtasks: Option<Vec<Task>>,     // Tasks can be hierarchically split into subtasks
    pub when: Option<NaiveDateTime>,     // The date you want to do the task
    pub deadline: Option<NaiveDateTime>, // The latest date the task should be done
    pub reminder: Option<NaiveDateTime>, // The datetime a reminder will alert you
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tasks {
    pub path: String,             // Where the tasks are stored
    pub tasks: Option<Vec<Task>>, // All the tasks in one vector
}

impl Task {
    pub fn new(title: String, notes: Option<String>, tags: Option<Vec<String>>,
               when: Option<NaiveDateTime>, deadline: Option<NaiveDateTime>,
               reminder: Option<NaiveDateTime>) -> Self {
        let status = if when.is_some() { Status::Pending } else { Status::Inbox };

        Self { title, status, notes, tags, subtasks: None, when, deadline, reminder, }
    }

    pub fn start(&mut self) {
        self.status = Status::Active;
    }

    pub fn stop(&mut self) {
        if self.when.is_none() {
            self.status = Status::Inbox;
        } else {
            self.status = Status::Pending;
        }
    }

    pub fn complete(&mut self) {
        self.status = Status::Complete;
    }
}

impl Tasks {
    pub fn new(tasks_path: &str) -> Self {
        Self {
            path: String::from(tasks_path),
            tasks: None
        }
    }

    fn task_not_found(&self, id: usize) -> TasksError {
        TasksError(format!("couldn't find task with id {}", id))
    }

    fn task_exists(&self, id: usize) -> bool{
        if id >= self.len() { false } else { true }
    }

    pub fn is_empty(&self) -> bool {
        if self.len() == 0 {
            true
        } else {
            false
        }
    }

    pub fn get_task(&mut self, id: usize) -> Result<&mut Task, TasksError> {
        if self.is_empty() {
            Err(TasksError(format!("no tasks available")))
        } else {
            if self.task_exists(id) {
                let task = &mut self.tasks.as_mut().unwrap()[id];
                Ok(task)
            } else {
                Err(TasksError(format!("couldn't find task with id {}", id)))
            }
        }
    }

    pub fn push(&mut self, task: Task) {
        if self.is_empty() {
            self.tasks = Some(vec![task]);
        } else {
            self.tasks.as_mut().unwrap().push(task);
        };
    }

    pub fn remove(&mut self, id: usize) -> Result<(), TasksError> {
        if self.task_exists(id) {
            self.tasks.as_mut().unwrap().remove(id);
            Ok(())
        } else {
            Err(self.task_not_found(id))
        }
    }

    pub fn len(&self) -> usize {
        if self.tasks.is_none() {
            0
        } else {
            self.tasks.as_ref().unwrap().len()
        }
    }

    pub fn clear(&mut self) -> Result<(), TasksError> {
        if self.is_empty() {
            Err(TasksError(String::from("no tasks available")))
        } else {
            self.tasks = None;
            Ok(())
        }
    }
}

impl Status {
    pub fn as_string(&self) -> ColoredString {
        match self {
            Status::Inbox => "📮 Inbox".blue(),
            Status::Pending => "📅 Pending".yellow(),
            Status::Active => "✍️ Active".red(),
            Status::Complete => "📗 Complete".green(),
        }
    }
}

