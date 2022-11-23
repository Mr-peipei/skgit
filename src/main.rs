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
use std::io::Cursor;
use std::io::{self, Write};
use std::process::Command;
struct CatItem {
    inner: String,
}

impl SkimItem for CatItem {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.inner)
    }

    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        let output = Command::new("cat")
            .arg(&self.inner)
            .output()
            .expect("something went wrong");
        ItemPreview::Command(format!("bat {}", self.inner))
        // // io::stdout().write_all(&output.stdout).unwrap()
        // if self.inner.starts_with("color") {
        //     ItemPreview::AnsiText(format!("\x1b[31mhello:\x1b[m\n{}", self.inner))
        // } else {
        //     ItemPreview::Text(format!("hello:\n{}", self.inner))
        // }
    }
}

pub fn main() {
    let options = SkimOptionsBuilder::default()
        .multi(true)
        .preview(Some("bat {} --color=always | sed 's/  */ /g'"))
        .build()
        .unwrap();

    let selected_items = Skim::run_with(&options, None)
        .map(|out| out.selected_items)
        .unwrap_or_else(Vec::new);

    for item in selected_items.iter() {
        let output = Command::new("cat")
            .arg(item.output().to_string())
            .output()
            .expect("something went wrong");
        io::stdout().write_all(&output.stdout).unwrap()
    }
}
