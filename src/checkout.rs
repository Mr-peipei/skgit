use git2::{Branch, BranchType, Repository};
use std::path::Path;

use crate::common::selected_branch_items;

pub fn checkout_branch() -> Result<(), git2::Error> {
    let repo = Repository::open(&Path::new("."))?;
    let branches = repo.branches(Some(BranchType::Local)).unwrap();

    for branch in branches {
        let bra = branch.unwrap().0;
        let branch_name = Branch::name(&bra).unwrap().unwrap();
        println!("{}", &branch_name);
    }

    let selected_branch = selected_branch_items(repo.branches(Some(BranchType::Local)).unwrap());

    println!("{}", selected_branch);

    Ok(())
}
