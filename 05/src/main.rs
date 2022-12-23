use std::io::{stdin, Read, BufRead, BufReader, Take};
use regex::Regex;

fn parse_crates<R>(stream: &mut R) -> Option<Vec<Vec<char>>>
where
    R: BufRead,
{
    let mut buffer = vec![0; 5];
    let mut scanlines: Vec<Vec<Option<char>>> = Vec::new();
    scanlines.push(Vec::new());
    let digit_re = Regex::new(r"^[0-9]$").unwrap();

    let mut handle;
    //println!("scanlines {} last {}", scanlines.len(), scanlines.last().unwrap().len());
    loop {
        handle = stream.take(3);
        handle.read(&mut buffer).unwrap();
        if buffer[0] == b' ' &&
            digit_re.is_match(&(buffer[1] as char).to_string())
        {
            //println!("breaking");
            scanlines.pop();
            break;
        }

        match buffer[1] {
            b' ' => {
                scanlines.last_mut().unwrap().push(None);
                //println!("found space");
            },
            x => {
                //println!("pushing {}", x as char);
                scanlines.last_mut().unwrap().push(Some(x as char));
            }
        }
        //println!("scanlines {} last {}", scanlines.len(), scanlines.last().unwrap().len());
        handle = stream.take(1);
        handle.read(&mut buffer).unwrap();
        //println!("break type:{}", buffer[0]);
        match buffer[0] {
            b'\n' => {
                scanlines.push(Vec::new());
                //println!("newline");
            },
            b' ' => {
                //println!("space")
            },
            _ => panic!("parse could not match expected character"),
        }
    }

    //println!("\nstack\n");
    //for row in scanlines.iter().rev() {
    //    let crate_char: char = match row[1] {
    //        Some(x) => x,
    //        None => ' ',
    //    };
    //    println!("{}", crate_char.to_string());
    //}

    // probably not the most efficient matrix mirroring in the world
    // but this does prune the empties
    let mut depot: Vec<Vec<char>> = Vec::new();
    depot.push(Vec::new());
    for stack in 0..scanlines[0].len() {
        for row in scanlines.iter().rev() {
            match row[stack] {
                Some(x) => depot.last_mut().unwrap().push(x),
                _ => {},
            }
        }
        depot.push(Vec::new());
    }

    //println!("\nstack");
    //for stacked_crate in &depot[1] {
    //    println!("{}", stacked_crate.to_string());
    //}

    return Some(depot);
}

fn main() {
    let mut depot = parse_crates(&mut stdin().lock()).unwrap();
    //for line in stdin().lock().lines() {
    //}
    println!("Hello, world!");
}
