use std::io::{stdin, Read, BufRead, BufReader};
use regex::Regex;

fn parse_crates<R>(stream: &mut R) -> Option<Vec<Vec<char>>>
where
    R: BufRead,
{
    let mut buffer = vec![0; 5];

    let mut handle = stream.take(3);
    handle.read(&mut buffer).unwrap();
    if buffer[1] == 32 {
        println!("found space");
    }
    handle = stream.take(1);
    handle.read(&mut buffer).unwrap();
    println!("{}", buffer[0]);
    match buffer[0] {
        b'\n' => println!("newline"),
        b' ' => println!("space"),
        _ => panic!("parse could not match expected character"),
    }
    if buffer[0] == 10 {
        
    }


    return Some(vec!(vec!('a','b')));
}

fn main() {
    let mut foo = parse_crates(&mut stdin().lock()).unwrap();
    //for line in stdin().lock().lines() {
    //}
    println!("Hello, world!");
}
