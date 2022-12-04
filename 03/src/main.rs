// Priority math:
// priorities are zip(/[a-zA-Z]/, 1..52)
// 'a' is 1, 'z' is 26, 'A' is 27, etc.
// So in char math:
// var (test.is_lower() ? test - 'a' : test - 'A') + 1

use std::io::{stdin, BufRead};

fn priority(x: u8) -> u32 {
    return (if (x as char).is_lowercase() { x - b'a' + 1 } else { x - b'A' + 27 }).into();
}

fn main() {
    let mut sum = 0;
    for line in stdin().lock().lines() {
        let mut ruck: Vec<u8> = line.unwrap().as_bytes().clone().to_vec();
        let half = ruck.len() / 2;
        ruck[..half].sort_by(|a, b| priority(*a).cmp(&priority(*b)));
        ruck[half..].sort_by(|a, b| priority(*a).cmp(&priority(*b)));
        //ruck(..half].sort();

        'outer: for i in ruck[..half].iter() {
            for j in ruck[half..].iter() {
                if i == j {
                    println!("read {} {} {} {half}", String::from_utf8_lossy(&ruck[..half]), String::from_utf8_lossy(&ruck[half..]), *i as char);
                    sum += priority(*i);
                    break 'outer;
                }
            }
        }
    }
    println!("{}", sum);
}
