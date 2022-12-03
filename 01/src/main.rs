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

    println!("max {}", max)
}
