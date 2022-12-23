use std::io::{stdin, Read, BufRead, BufReader, Take};
use regex::Regex;

fn parse_crates<R>(stream: &mut R) -> Option<Vec<Vec<char>>>
where
    R: BufRead,
{
    let mut buffer = vec![0; 5];
    let mut scanlines: Vec<Vec<Option<u8>>> = Vec::new();
    scanlines.push(Vec::new());
    let mut stack = 0;
    let mut row = 0;
    let digit_re = Regex::new(r"^[0-9]$").unwrap();

    let mut handle;
    loop {
        handle = stream.take(3);
        handle.read(&mut buffer).unwrap();
        if buffer[0] == b' ' &&
            digit_re.is_match(&(buffer[1] as char).to_string())
        {
            println!("breaking");
            break;
        }
        match buffer[1] {
            b' ' => {
                scanlines[row].push(None);
                println!("found space");
            },
            x => {
                println!("pushing {}", x as char);
                scanlines[row].push(Some(x));
            }
        }
        handle = stream.take(1);
        handle.read(&mut buffer).unwrap();
        println!("{}", buffer[0]);
        match buffer[0] {
            b'\n' => {
                scanlines.push(Vec::new());
                println!("newline");
            },
            b' ' => println!("space"),
            _ => panic!("parse could not match expected character"),
        }
    }


    return Some(vec!(vec!('a','b')));
}

fn main() {
    let mut foo = parse_crates(&mut stdin().lock()).unwrap();
    //for line in stdin().lock().lines() {
    //}
    println!("Hello, world!");
}
