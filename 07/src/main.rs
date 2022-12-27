use std::rc::Rc;
use std::io::{stdin, BufRead};
use regex::Regex;

enum FileType {
    Directory,
    File,
}

struct INode {
    parent: Option<Rc<INode>>,
    name: String,
    kind: FileType,
    size: usize,
    children: Vec<INode>,
}

fn parse_file_tree<T>(stream: &mut T) -> Option<Box<INode>>
where
    T: BufRead,
{
    let command_regex: Regex = Regex::new(r"\$ ([[:alpha:]]+) ([[:alpha:]]+)").ok()?;

    let tree: Box<INode> = Box::new(INode {
        parent: None,
        name: "/".to_string(),
        kind: FileType::Directory,
        size: 0,
        children: Vec::new(),
    });
    let mut cursor: Rc<INode>;
    for line_res in stream.lines() {
        let line = line_res.ok()?;
        if command_regex.is_match(&line) {
            let cap = command_regex.captures(&line)?;
            match cap.get(1)?.as_str() {
                "cd" => {
                    let directory = cap.get(2)?.as_str();
                    match directory {
                        "/" => cursor = Rc::new(*tree),
                        ".." => cursor = Rc::clone(&cursor.parent?),
                        dir => cursor = Rc::new(cursor.children.iter().find(|&x| x.name == dir ).unwrap()),
                    }
                },
                _ => todo!(),
            }
        }
    }
    None
}

fn main() {
    let mut stream = stdin().lock();
    let file_tree = parse_file_tree(&mut stream);
}
