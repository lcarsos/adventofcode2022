use std::io::{stdin, Read, BufRead, BufReader};
use regex::Regex;

fn parse_crates(stream: &mut dyn Read) -> Vec<Vec<char>> {
    let mut buffer = [0; 5];

    let mut handle = stream.take(3);
    handle.read(&mut buffer).unwrap();
    return vec!(vec!('a','b'));
}

fn main() {
    let mut foo = parse_crates(&mut stdin().lock());
    //for line in stdin().lock().lines() {
    //}
    println!("Hello, world!");
}
