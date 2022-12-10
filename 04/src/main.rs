use std::io::{stdin, BufRead};
use regex::Regex;

#[derive(Default)]
struct Range {
    left: u64,
    right: u64
}

impl Range {
    fn contains(&self, &other: &Range) -> bool {
        return self.left <= other.left && self.right >= other.right;
    }
}

fn main() {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    for line in stdin().lock().lines() {
        let st = line.unwrap();
        let cap = re.captures(&st).unwrap();
        let range_a = Range { left: cap[1].parse().unwrap(), right: cap[2].parse().unwrap()};
        let range_b = Range { left: cap[3].parse().unwrap(), right: cap[4].parse().unwrap()};
        let answer = range_a.contains(&range_b);

        println!("a contains b: {}", answer);
        println!("{} {}", range_a.left, range_a.right);
        break;
    }
}
