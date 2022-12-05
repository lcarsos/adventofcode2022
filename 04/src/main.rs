use std::io::{stdin, BufRead};
use regex::Regex;

#[derive(Default)]
struct Range {
    left: u64,
    right: u64
}

fn main() {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    for line in stdin().lock().lines() {
        let st = line.unwrap();
        let cap = re.captures(&st).unwrap();
        let range_a = Range { left: cap[1].parse().unwrap(), right: cap[2].parse().unwrap()};
        //let nibbleA: u64 = cap[1].parse().unwrap();
        //let nibbleB: u64 = cap[2].parse().unwrap();
        println!("{} {}", range_a.left, range_a.right);
        break;
    }
}
