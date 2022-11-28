use git2::{Repository, StatusOptions};
use std::path::Path;

use crate::common::{selected_items, status_list};

pub fn add_files() -> Result<(), git2::Error> {
    let repo = Repository::open(&Path::new("."))?;

    let mut opts = StatusOptions::new();
    opts.include_untracked(true);
    let statuses = repo.statuses(Some(&mut opts))?;

    let status_list = status_list(&repo, &statuses);

    let mut index = repo.index()?;

    let selected_files = selected_items(status_list);

    for item in selected_files.iter() {
        println!("added {}", &item.to_str().unwrap());
        println!("added {}", &item.to_str().unwrap());
        index.add_path(item.as_path())?;
    }

    index.write()?;
    Ok(())
}
