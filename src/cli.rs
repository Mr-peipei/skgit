use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Git add command.
    Add,
    /// Git checkout command.
    Checkout,
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
