use git2::Repository;
use std::path::Path;

pub fn add_files(args: &Vec<String>) -> Result<(), git2::Error> {
    let repo = Repository::open(&Path::new("."))?;
    let mut index = repo.index()?;

    let cb = &mut |path: &Path, _matched_spec: &[u8]| -> i32 {
        let status = repo.status_file(path).unwrap();
        println!("add '{}'", path.display());

        let ret = if status.contains(git2::Status::WT_MODIFIED)
            || status.contains(git2::Status::WT_NEW)
        {
            println!("add '{}'", path.display());
            0
        } else {
            println!("add '{}'", path.display());
            1
        };

        if false {
            println!("add '{}'", path.display());
            1
        } else {
            println!("add '{}'", path.display());
            ret
        }
    };
    let cb = if false || false {
        println!("here1");
        Some(cb as &mut git2::IndexMatchedPath)
    } else {
        println!("here2");
        None
    };

    if false {
        index.update_all(vec![""].iter(), cb)?;
    } else {
        println!("here3");
        index.add_all(args.iter(), git2::IndexAddOption::DEFAULT, None)?;
    }

    index.write()?;
    Ok(())
}
