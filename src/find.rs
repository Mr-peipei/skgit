use std::{borrow::Cow, process::Command, sync::Arc};

use skim::{
    prelude::{unbounded, SkimOptionsBuilder},
    ItemPreview, PreviewContext, Skim, SkimItem, SkimItemReceiver, SkimItemSender,
};

struct StatusItem {
    inner: String,
}

impl SkimItem for StatusItem {
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

pub fn selected_items(status_list: Vec<String>) -> Vec<String> {
    let options = SkimOptionsBuilder::default()
        .multi(true)
        .preview(Some("bat {} --color=always | sed 's/  */ /g'"))
        .build()
        .unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();

    for str in status_list {
        println!("{}", str);
        let _ = tx_item.send(Arc::new(StatusItem { inner: str }));
    }
    drop(tx_item);

    let selected_items = Skim::run_with(&options, Some(rx_item))
        .map(|out| out.selected_items)
        .unwrap_or_else(Vec::new);

    // Replace "./" to "" , because of add_all cannot be used by including "./" paths.
    let selected_files: Vec<String> = selected_items
        .iter()
        .map(|x| format_str(x.output().to_string()))
        .rev()
        .collect();

    selected_files
}

fn format_str(str: String) -> String {
    let mut line = str.split_whitespace();
    let one = line.next();
    let two = line.next();

    return match two {
        Some(n) => return n.to_string(),
        None => "".to_string(),
    };
}
