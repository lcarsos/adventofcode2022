use std::io::{stdin, BufRead};
//use std::io::BufReader;

fn main() {
    let mut inventory = Vec::new();
    inventory.push(Vec::new());

    for input in stdin().lock().lines() {
        //println!("raw read: {}", input.unwrap());
        match input.unwrap().parse::<i32>() {
            Ok(x) => inventory.last_mut().unwrap().push(x),
            Err(_) => inventory.push(Vec::new()),
        }
    }

    //let out: String = inventory[0][0].to_string();
    println!("{}", inventory.len());
    println!("{}", inventory[0].len());

}
