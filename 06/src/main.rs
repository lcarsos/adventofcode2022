use std::io::{stdin, BufRead};

fn main() {
    let input = stdin().lock().lines();
    let line = input.unwrap().unwrap();
    println!("Hello, world!");
}
