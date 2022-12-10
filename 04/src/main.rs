use std::io::{stdin, BufRead};
use regex::Regex;

#[derive(Default)]
struct Range {
    left: u64,
    right: u64
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        return self.left <= other.left && self.right >= other.right;
    }
}

fn main() {
    let mut count = 0;
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    for line in stdin().lock().lines() {
        let st = line.unwrap();
        let cap = re.captures(&st).unwrap();
        let range_a = Range { left: cap[1].parse().unwrap(), right: cap[2].parse().unwrap()};
        let range_b = Range { left: cap[3].parse().unwrap(), right: cap[4].parse().unwrap()};
        if (range_a.contains(&range_b) || range_b.contains(&range_a)) {
            count += 1;
        }
    }
    println!("count: {}", count);
}
