use std::rc::Rc;
use std::cell::RefCell;
use std::io::{stdin, BufRead};
use regex::Regex;

type RcINode = Rc<RefCell<INode>>;

enum Command {
    ChangeDir,
    List,
}

enum FileType {
    Directory,
    File,
}

struct INode {
    parent: Option<RcINode>,
    name: String,
    kind: FileType,
    size: usize,
    children: Vec<RcINode>,
}

fn parse_file_tree<T>(stream: &mut T) -> Option<RcINode>
where
    T: BufRead,
{
    let command_regex: Regex = Regex::new(r"\$ ([[:alpha:]]+) ([[:alpha:]]+)").ok()?;

    let tree: RcINode = Rc::new(RefCell::new(INode {
        parent: None,
        name: String::from("/"),
        kind: FileType::Directory,
        size: 0,
        children: Vec::new(),
    }));
    let mut cursor: RcINode;
    for line_res in stream.lines() {
        parse_command(Rc::clone(&tree), line_res
        let line = line_res.ok()?;
        if command_regex.is_match(&line) {
            let cap = command_regex.captures(&line)?;
            match cap.get(1)?.as_str() {
                "cd" => {
                    let directory = cap.get(2)?.as_str();
                    match directory {
                        "/" => cursor = Rc::clone(&tree),
                        ".." => cursor = Rc::clone(&cursor.borrow().parent?),
                        //dir => cursor = Rc::clone(&*cursor.borrow().children.iter().find(|&x| x.name == dir ).unwrap()),
                        _ => todo!(),
                    }
                },
                _ => todo!(),
            }
        }
    }
    None
}

fn parse_command<T>(current: RcINode, stream: &mut T) -> RcINode
where
    T: BufRead
{
}



fn main() {
    let mut stream = stdin().lock();
    let file_tree = parse_file_tree(&mut stream);
}
