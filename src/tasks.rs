use chrono::{Local, NaiveDateTime};
use colored::{ColoredString, Colorize};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct TasksError(String);

impl TasksError {
    pub fn no_task(id: usize) -> Self {
        Self(format!("couldn't find task with id {}", id))
    }

    pub fn no_tasks() -> Self {
        Self(String::from("no tasks available"))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Inbox,    // When you create a new task without a when date
    Pending,  // When you give a task a when date
    Active,   // When you have started a task
    Complete, // When a task is completed
}

impl Status {
    pub fn as_colored_string(&self) -> ColoredString {
        match self {
            Status::Inbox => "📮 Inbox".blue(),
            Status::Pending => "📅 Pending".yellow(),
            Status::Active => "🕑 Active".red(),
            Status::Complete => "📗 Complete".green(),
        }
    }
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

    pub fn modify(
        &mut self,
        title: Option<String>,
        notes: Option<String>,
        tags: Option<Vec<String>>,
        when: Option<NaiveDateTime>,
        deadline: Option<NaiveDateTime>,
        reminder: Option<NaiveDateTime>,
    ) {
        self.title = title.unwrap_or_else(|| self.title.clone());
        self.notes = notes.or_else(|| self.notes.take());
        self.tags = tags.or_else(|| self.tags.take());
        self.when = when.or_else(|| self.when.take());
        self.deadline = deadline.or_else(|| self.deadline.take());
        self.reminder = reminder.or_else(|| self.reminder.take());

        if let Some(_when) = self.when {
            if self.is_inbox() {
                self.status = Status::Pending;
            }
        }
    }
}

impl Task {
    pub fn inbox(&mut self) {
        self.status = Status::Inbox;
        self.when = None;
    }

    pub fn complete(&mut self) {
        self.status = Status::Complete;
    }

    pub fn start(&mut self) {
        self.status = Status::Active;
    }

    pub fn pend(&mut self) {
        self.status = Status::Pending;
    }

    pub fn stop(&mut self) {
        if self.when.is_some() {
            self.status = Status::Pending;
        } else {
            self.status = Status::Inbox;
        }
    }
}

impl Task {
    pub fn is_complete(&self) -> bool {
        self.status == Status::Complete
    }

    pub fn is_active(&self) -> bool {
        self.status == Status::Active
    }

    pub fn is_pending(&self) -> bool {
        self.status == Status::Pending
    }

    pub fn is_inbox(&self) -> bool {
        self.status == Status::Inbox
    }
}

impl Task {
    fn date_colored_string(&self, date: &Option<NaiveDateTime>) -> ColoredString {
        if let Some(date) = date {
            let date = date.date();
            let date_string = format!("{}", date.format("%Y-%m-%d"));
            let now = Local::now().date_naive();

            if date <= now {
                // If the date is today or past today
                date_string.bright_red()
            } else if now.succ_opt().unwrap() == date {
                // If the date is tomorrow
                date_string.yellow()
            } else {
                // Otherwise the date is too far in the past
                date_string.white()
            }
        } else {
            // No date available
            "N/A".bright_black()
        }
    }

    pub fn when_string(&self) -> ColoredString {
        self.date_colored_string(&self.when)
    }

    pub fn deadline_string(&self) -> ColoredString {
        self.date_colored_string(&self.deadline)
    }

    pub fn reminder_string(&self) -> ColoredString {
        self.date_colored_string(&self.reminder)
    }

    pub fn title_string(&self) -> ColoredString {
        self.title.white()
    }

    pub fn status_string(&self) -> ColoredString {
        self.status.as_colored_string()
    }

    pub fn tags_string(&self) -> ColoredString {
        if let Some(tags) = &self.tags {
            tags.join(", ").white()
        } else {
            "N/A".bright_black()
        }
    }

    pub fn notes_string(&self) -> ColoredString {
        if let Some(notes) = &self.notes {
            notes.white()
        } else {
            "N/A".bright_black()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tasks {
    pub path: String,             // Path to the tasks repository
    pub file: String,             // Path to the tasks file in the repository
    pub tasks: Vec<Task>, // All the tasks in one vector
}

impl Tasks {
    pub fn new(repo_path: &str, tasks_file: &str) -> Self {
        Self {
            path: String::from(repo_path),
            file: String::from(tasks_file),
            tasks: Vec::new(),
        }
    }
}

impl Tasks {
    /// Checks if tasks are empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Checks if a task exists from an id
    pub fn exists(&self, id: usize) -> bool {
        id < self.len()
    }

    /// Returns a task from an id
    pub fn task(&mut self, id: usize) -> Result<&mut Task, TasksError> {
        if self.is_empty() {
            Err(TasksError::no_tasks())
        } else if self.exists(id) {
            Ok(&mut self.tasks[id])
        } else {
            Err(TasksError::no_task(id))
        }
    }
}

impl Tasks {
    pub fn push(&mut self, task: Task) {
        if self.is_empty() {
            self.tasks = vec![task];
        } else {
            self.tasks.push(task);
        };
    }

    pub fn remove(&mut self, id: usize) -> Result<(), TasksError> {
        if self.exists(id) {
            self.tasks.remove(id);
            Ok(())
        } else {
            Err(TasksError::no_task(id))
        }
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    pub fn clear(&mut self) -> Result<(), TasksError> {
        if self.is_empty() {
            Err(TasksError::no_tasks())
        } else {
            self.tasks = Vec::new();
            Ok(())
        }
    }
}
