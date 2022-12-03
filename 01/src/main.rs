use std::io::{stdin, BufRead};
//use std::io::BufReader;

fn main() {
    let mut inventory = Vec::new();
    inventory.push(Vec::new());

    let mut max = 0;
    let mut calories = 0;
    for input in stdin().lock().lines() {
        match input.unwrap().parse::<i32>() {
            Ok(x) => {
                inventory.last_mut().unwrap().push(x);
                calories += x;
            },
            Err(_) => {
                if calories > max { max = calories; }
                calories = 0;
                inventory.push(Vec::new());
            },
        }
    }

    let mut sums: Vec<i32> = inventory.iter().map(|elf| elf.iter().sum()).collect();
    sums.sort();
    sums.reverse();
    let top_three: i32 = sums[0..3].iter().sum();

    println!("max {}", max);
    println!("top three {}", top_three);
}
