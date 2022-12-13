use std::io::{stdin, BufRead, BufReader};
use regex::Regex;

fn parse_crates(stream: BufReader<R>) -> Vec<Vec<char>> {
    let mut buffer = [0; 5];

    let mut handle = stream.take(3);
    handle.read(&mut buffer);
    return vec!(vec!('a','b'));
}

fn main() {
    let mut foo = parse_crates(stdin().lock());
    //for line in stdin().lock().lines() {
    //}
    println!("Hello, world!");
}
