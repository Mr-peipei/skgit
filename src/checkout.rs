use git2::{Repository, StatusOptions};
use std::path::Path;

use crate::common::{selected_items, status_list};

pub fn checkout_branch() -> Result<(), git2::Error> {
    let repo = Repository::open(&Path::new("."))?;

    Ok(())
}
