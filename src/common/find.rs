use std::{borrow::Cow, path::PathBuf, sync::Arc};

use skim::{
    prelude::{unbounded, Key, SkimOptionsBuilder},
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
        // Override Preview Func
        ItemPreview::Command(format!(
            "git diff --color=always --minimal {} ",
            format_str(self.inner.to_string())
        ))
    }
}

pub fn selected_items(status_list: Vec<String>) -> Vec<PathBuf> {
    let options = SkimOptionsBuilder::default()
        .multi(true)
        .preview(Some(""))
        .build()
        .unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for str in status_list {
        let _ = tx_item.send(Arc::new(StatusItem { inner: str }));
    }
    drop(tx_item);

    let selected_items = Skim::run_with(&options, Some(rx_item))
        .map(|out| match out.final_key {
            Key::Enter => out.selected_items,
            _ => Vec::new(),
        })
        .unwrap();

    let selected_files: Vec<PathBuf> = selected_items
        .iter()
        .map(|x| format_path_buf(x.output().to_string()))
        .rev()
        .collect();

    selected_files
}

fn format_path_buf(str: String) -> PathBuf {
    let mut line = str.split_whitespace();
    line.next();
    let two = line.next();

    return match two {
        Some(n) => return PathBuf::from(&n.to_string()),
        None => PathBuf::from(&"".to_string()),
    };
}

fn format_str(str: String) -> String {
    let mut line = str.split_whitespace();
    line.next();
    let two = line.next();

    return match two {
        Some(n) => return n.to_string(),
        None => "".to_string(),
    };
}
