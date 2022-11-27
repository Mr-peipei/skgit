use git2::{Repository, StatusOptions};
use std::path::Path;

use crate::{find::selected_items, status::status_list};

pub fn add_files() -> Result<(), git2::Error> {
    let repo = Repository::open(&Path::new("."))?;

    let mut opts = StatusOptions::new();
    opts.include_untracked(true);
    let statuses = repo.statuses(Some(&mut opts))?;

    // print_long(&statuses);
    let status_list = status_list(&repo, &statuses);

    let mut index = repo.index()?;

    let cb = &mut |path: &Path, _matched_spec: &[u8]| -> i32 {
        let status = repo.status_file(path).unwrap();
        println!("add '{}'", path.display());

        let ret = if status.contains(git2::Status::WT_MODIFIED)
            || status.contains(git2::Status::WT_NEW)
        {
            0
        } else {
            1
        };

        if false {
            1
        } else {
            ret
        }
    };
    let cb = if false || false {
        Some(cb as &mut git2::IndexMatchedPath)
    } else {
        None
    };
    let selected_files = selected_items(status_list);

    if false {
        index.update_all(vec![""].iter(), cb)?;
    } else {
        index.add_all(selected_files.iter(), git2::IndexAddOption::DEFAULT, None)?;
    }

    index.write()?;
    Ok(())
}
