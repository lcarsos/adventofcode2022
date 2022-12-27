use std::io::{stdin, BufRead};
use regex::Regex;

enum FileType {
    Directory,
    File,
}

struct INode {
    parent: Option<Box<INode>>,
    kind: FileType,
    size: usize,
    children: Vec<INode>,
}

fn parse_file_tree<T>(stream: &mut T) -> Option<Box<INode>>
where
    T: BufRead,
{
    let command_regex: Regex = Regex::new(r"\$ ([[:alpha:]]+) ([[:alpha:]]+)")?;

    let mut tree: Box<INode>;
    let mut cursor: &mut INode;
    for line_res in stream.lines() {
        let line = line_res?;
        if command_regex.is_match(&line) {
            let cap = command_regex.captures(&line)?;
            match cap.get(1)?.text {
                "cd" => {

                },
            }
        }
    }
    None
}

fn main() {
    let mut stream = stdin().lock();
    let file_tree = parse_file_tree(&mut stream);
}
