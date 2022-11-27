extern crate skim;
mod add;
mod command;
mod find;

use structopt::StructOpt;
mod cli;
mod status;
mod tasks;

use cli::{Action::*, CommandLineArgs};

pub fn main() -> anyhow::Result<()> {
    let CommandLineArgs { action } = CommandLineArgs::from_args();

    // Perform the action.
    match action {
        Add => add::add_files(),
    }?;

    Ok(())
}
