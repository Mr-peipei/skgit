extern crate skim;
mod add;
mod checkout;
mod common;

use structopt::StructOpt;
mod cli;

use cli::{Action::*, CommandLineArgs};

pub fn main() -> anyhow::Result<()> {
    let CommandLineArgs { action } = CommandLineArgs::from_args();

    // Perform the action.
    match action {
        Add => add::add_files(),
        Checkout => checkout::checkout_branch(),
    }?;

    Ok(())
}
