use chrono::NaiveDateTime;
use colored::*;
use serde::{Deserialize, Serialize};

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
    pub title: String,                   // The required title of the task
    pub status: Status,                  // Current status of the task
    pub notes: Option<String>,           // Any notes to explain the task
    pub tags: Option<Vec<String>>,       // Tasks can be tagged for organisation
    pub when: Option<NaiveDateTime>,     // The date you want to do the task
    pub deadline: Option<NaiveDateTime>, // The latest date the task should be done
    pub reminder: Option<NaiveDateTime>, // The datetime a reminder will alert you
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tasks {
    pub path: String,             // Where the tasks are stored
    pub tasks: Option<Vec<Task>>, // All the tasks in one vector
}

fn task_not_found(id: usize) -> TasksError {
    TasksError(format!("couldn't find task with id {}", id))
}

fn no_tasks_available() -> TasksError {
    TasksError(String::from("no tasks available"))
}

impl Task {
    pub fn new(
        title: String,
        notes: Option<String>,
        tags: Option<Vec<String>>,
        when: Option<NaiveDateTime>,
        deadline: Option<NaiveDateTime>,
        reminder: Option<NaiveDateTime>,
    ) -> Self {
        let status = if when.is_some() {
            Status::Pending
        } else {
            Status::Inbox
        };

        Self {
            title,
            status,
            notes,
            tags,
            when,
            deadline,
            reminder,
        }
    }

    pub fn modify(&mut self, title: Option<String>, notes: Option<String>, tags: Option<Vec<String>>, when: Option<NaiveDateTime>, deadline: Option<NaiveDateTime>, reminder: Option<NaiveDateTime>) {
        if title.is_some() {
            self.title = title.unwrap();
        };

        if notes.is_some() {
            self.notes = Some(notes.unwrap());
        };

        if tags.is_some() {
            self.tags = Some(tags.unwrap());
        };

        if when.is_some() {
            self.when = Some(when.unwrap());
        };

        if deadline.is_some() {
            self.deadline = Some(deadline.unwrap());
        };

        if reminder.is_some() {
            self.reminder = Some(reminder.unwrap());
        };
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
            tasks: None,
        }
    }

    pub fn task_exists(&self, id: usize) -> bool {
        if id >= self.len() {
            false
        } else {
            true
        }
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
            Err(no_tasks_available())
        } else {
            if self.task_exists(id) {
                Ok(&mut self.tasks.as_mut().unwrap()[id])
            } else {
                Err(task_not_found(id))
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
            Err(task_not_found(id))
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
            Err(no_tasks_available())
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
            Status::Active => "🕑 Active".red(),
            Status::Complete => "📗 Complete".green(),
        }
    }
}
