// use std::path::PathBuf;

// use structopt::StructOpt;
// mod cli;
// mod tasks;

// use cli::{Action::*, CommandLineArgs};
// use tasks::Task;

// fn find_default_journal_file() -> Option<PathBuf> {
//     home::home_dir().map(|mut path| {
//         path.push(".rusty-journal.json");
//         path
//     })
// }

// fn main() -> anyhow::Result<()> {
//     // Get the command-line arguments.
//     let CommandLineArgs {
//         action,
//         journal_file,
//     } = CommandLineArgs::from_args();

//     // Unpack the journal file.
//     let journal_file = journal_file
//         .or_else(find_default_journal_file)
//         .expect("Failed to find journal file");

//     // Perform the action.
//     match action {
//         Add { text } => tasks::add_task(journal_file, Task::new(text)),
//         List => tasks::list_tasks(journal_file),
//         Done { position } => tasks::complete_task(journal_file, position),
//     }?;
//     Ok(())
// }
extern crate skim;
use skim::prelude::*;
use std::process::Command;

pub fn main() {
    let options = SkimOptions::default();

    let selected_items = Skim::run_with(&options, None)
        .map(|out| out.selected_items)
        .unwrap_or_else(Vec::new);

    for item in selected_items.iter() {
        Command::new("cat")
            .arg(item.output().to_string())
            .spawn()
            .expect("something went wrong");
    }
}
