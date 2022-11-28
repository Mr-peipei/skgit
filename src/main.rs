extern crate skim;
mod add;
mod common;

use structopt::StructOpt;
mod cli;

use cli::{Action::*, CommandLineArgs};

pub fn main() -> anyhow::Result<()> {
    let CommandLineArgs { action } = CommandLineArgs::from_args();

    // Perform the action.
    match action {
        Add => add::add_files(),
    }?;

    Ok(())
}
