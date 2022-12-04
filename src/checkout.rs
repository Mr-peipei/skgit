use git2::{Branch, BranchType, Repository};
use std::path::Path;

pub fn checkout_branch() -> Result<(), git2::Error> {
    let repo = Repository::open(&Path::new("."))?;
    let branches = repo.branches(Some(BranchType::Local)).unwrap();

    for branch in branches {
        let bra = branch.unwrap().0;
        let branch_name = Branch::name(&bra).unwrap().unwrap();
        println!("{}", &branch_name);
    }

    Ok(())
}
