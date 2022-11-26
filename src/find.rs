use std::{borrow::Cow, process::Command};

use skim::{prelude::SkimOptionsBuilder, ItemPreview, PreviewContext, Skim, SkimItem};

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
    }
}

pub fn selected_items() -> Vec<String> {
    let options = SkimOptionsBuilder::default()
        .multi(true)
        .preview(Some("bat {} --color=always | sed 's/  */ /g'"))
        .build()
        .unwrap();

    let selected_items = Skim::run_with(&options, None)
        .map(|out| out.selected_items)
        .unwrap_or_else(Vec::new);

    // Replace "./" to "" , because of add_all cannot be used by including "./" paths.
    let selected_files: Vec<String> = selected_items
        .iter()
        .map(|x| x.output().to_string().replace("./", ""))
        .rev()
        .collect();

    for item in selected_files.iter() {
        println!("{}", &item)
    }

    selected_files
}
