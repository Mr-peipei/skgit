use git2::{BranchType, Repository};
use std::path::Path;

use crate::common::selected_branch_items;

pub fn checkout_branch() -> Result<(), git2::Error> {
    let repo = Repository::open(&Path::new("."))?;

    let selected_branch = selected_branch_items(repo.branches(Some(BranchType::Local)).unwrap());

    println!("{}", selected_branch);

    let str = String::from("refs/heads/") + &selected_branch;
    repo.set_head(&str).unwrap();

    Ok(())
}
