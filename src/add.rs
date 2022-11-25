use git2::Repository;
use std::path::Path;

pub fn add_files(args: &Vec<String>) -> Result<(), git2::Error> {
    let repo = Repository::open(&Path::new("."))?;
    let mut index = repo.index()?;

    index.add_all(args.into_iter(), git2::IndexAddOption::DEFAULT, None)?;

    index.write()?;
    Ok(())
}
