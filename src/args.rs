use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct TasksArgs {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, PartialEq, Eq, Debug)]
pub enum Commands {
    /// Creates a new task
    Add(CreateTask),
    /// Deletes a task without completing it
    Del(DeleteTask),
    /// Deletes all tasks
    Clear,
    /// Shows info about a task
    Show(ShowTask),
    /// Marks a task as completed
    Done(CompleteTask),
    /// Marks a task as active
    Start(StartTask),
    /// Marks a task as pending
    Stop(StopTask),
    /// Returns a task to the inbox
    Inbox(InboxTask),
    /// Edit a task with $EDITOR
    Edit(EditTask),
    /// Modify a task at the command line
    Modify(ModifyTask),
    /// Passes git commands to the repository
    Git(GitExecute),
    /// Pull then push to git remote and merge commits
    Sync(SyncTasks),
    /// Undo a number of commits
    Undo(UndoExecute),
}

#[derive(Args, PartialEq, Eq, Debug)]
pub struct CreateTask {
    /// Title of the task
    pub title: String,

    /// Any notes to help explain/remember the task
    #[arg(short, long)]
    #[clap(default_value=None)]
    pub notes: Option<String>,

    /// Tags for organisation, separated by commas
    #[arg(short, long)]
    #[clap(default_value=None)]
    pub tags: Option<String>,

    /// Date when you want to do the task
    #[arg(short, long)]
    #[clap(default_value=None)]
    pub when: Option<String>,

    /// Deadline when the task has to be in
    #[arg(short, long)]
    #[clap(default_value=None)]
    pub deadline: Option<String>,

    /// The date and time when you want to be reminded
    #[arg(short, long)]
    #[clap(default_value=None)]
    pub reminder: Option<String>,
}
#[derive(Args, PartialEq, Eq, Debug)]
pub struct DeleteTask {
    /// ID of the task
    pub id: usize,
}
#[derive(Args, PartialEq, Eq, Debug)]
pub struct ShowTask {
    /// ID of the task
    #[clap(default_value=None)]
    pub id: Option<usize>,
}
#[derive(Args, PartialEq, Eq, Debug)]
pub struct CompleteTask {
    /// ID of the task
    pub id: usize,
}
#[derive(Args, PartialEq, Eq, Debug)]
pub struct StartTask {
    /// ID of the task
    pub id: usize,
}
#[derive(Args, PartialEq, Eq, Debug)]
pub struct StopTask {
    /// ID of the task
    pub id: usize,
}
#[derive(Args, PartialEq, Eq, Debug)]
pub struct InboxTask {
    /// ID of the task
    pub id: usize,
}
#[derive(Args, PartialEq, Eq, Debug)]
pub struct EditTask {
    /// ID of the task
    pub id: usize,
}
#[derive(Args, PartialEq, Eq, Debug)]
pub struct ModifyTask {
    /// ID of the task
    pub id: usize,

    /// Title of the task
    #[clap(default_value=None)]
    pub title: Option<String>,

    /// Any notes to help explain/remember the task
    #[arg(short, long)]
    #[clap(default_value=None)]
    pub notes: Option<String>,

    /// Tags for organisation, separated by commas
    #[arg(short, long)]
    #[clap(default_value=None)]
    pub tags: Option<String>,

    /// Date when you want to do the task
    #[arg(short, long)]
    #[clap(default_value=None)]
    pub when: Option<String>,

    /// Deadline when the task has to be in
    #[arg(short, long)]
    #[clap(default_value=None)]
    pub deadline: Option<String>,

    /// The date and time when you want to be reminded
    #[arg(short, long)]
    #[clap(default_value=None)]
    pub reminder: Option<String>,
}
#[derive(Args, PartialEq, Eq, Debug)]
pub struct GitExecute {
    /// Git command to run
    pub command: String,
}
#[derive(Args, PartialEq, Eq, Debug)]
pub struct SyncTasks {
    /// Git remote to use
    #[clap(default_value = "origin")]
    pub remote: String,
}
#[derive(Args, PartialEq, Eq, Debug)]
pub struct UndoExecute {
    /// Number of times to undo
    #[clap(default_value = "1")]
    pub number: String,
}
