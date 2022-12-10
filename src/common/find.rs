use std::{borrow::Cow, path::PathBuf, sync::Arc};

use git2::{Branch, Branches};
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
            get_file_name(self.inner.to_string())
        ))
    }
}

pub fn selected_file_items(status_list: Vec<String>) -> Vec<PathBuf> {
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
        .map(|x| string_to_path(x.output().to_string()))
        .rev()
        .collect();

    selected_files
}

pub fn selected_branch_items(status_list: Branches) -> String {
    let options = SkimOptionsBuilder::default().build().unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for str in status_list {
        let _ = tx_item.send(Arc::new(StatusItem {
            inner: Branch::name(&str.unwrap().0).unwrap().unwrap().to_string(),
        }));
    }
    drop(tx_item);

    let selected_item = Skim::run_with(&options, Some(rx_item))
        .map(|out| out.selected_items)
        .unwrap_or_else(Vec::new);

    let mut selected_branch: Vec<String> = selected_item
        .iter()
        .map(|x| x.output().to_string())
        .rev()
        .collect();

    let branch = selected_branch.pop().unwrap();

    branch
}

fn string_to_path(str: String) -> PathBuf {
    let mut line = str.split_whitespace();
    line.next();
    let two = line.next();

    return match two {
        Some(n) => return PathBuf::from(&n.to_string()),
        None => PathBuf::from(&"".to_string()),
    };
}

fn get_file_name(str: String) -> String {
    let mut line = str.split_whitespace();
    line.next();
    let two = line.next();

    return match two {
        Some(n) => return n.to_string(),
        None => "".to_string(),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_to_path_test() {
        assert_eq!(
            string_to_path(String::from("MM filename.txt")),
            PathBuf::from("filename.txt")
        );
        assert_eq!(
            string_to_path(String::from("MM filename.txt")),
            PathBuf::from("filename.txt")
        )
    }

    #[test]
    fn format_str_test() {
        assert_eq!(
            get_file_name(String::from("MM filename.txt")),
            "filename.txt"
        );
        assert_eq!(
            get_file_name(String::from("MM filename.txt")),
            "filename.txt"
        )
    }
}
