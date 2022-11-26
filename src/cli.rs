use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Write tasks to the journal file.
    Add,
    // / Remove an entry from the journal file by position.
    // Done {
    //     #[structopt()]
    //     position: usize,
    // },
    // /// List all tasks in the journal file.
    // List,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rusty Journal",
    about = "A command line to-do app written in Rust"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
}
