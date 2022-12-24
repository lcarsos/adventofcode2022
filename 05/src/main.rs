use std::io::{stdin, Read, BufRead, BufReader};
use regex::Regex;

type Depot = Vec<Vec<char>>;

struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

fn print_depot(depot: &Depot) {
    for (i, stack) in depot.iter().enumerate() {
        print!("{} |", i+1);
        for j in stack.iter() {
            print!(" {j}");
        }
        print!("\n");
    }
}

fn print_depot_top(depot: &Depot) {
    for i in depot.iter() {
        print!("{}", i.last().unwrap());
    }
    print!("\n");
}

fn parse_crates<R>(stream: &mut R) -> Option<Depot>
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
            // every time we read in a newline we push a new vec
            // but the last vec is unnecessary so we pop it off
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
    for stack in 0..scanlines[0].len() {
        depot.push(Vec::new());
        for row in scanlines.iter().rev() {
            match row[stack] {
                Some(x) => depot.last_mut().unwrap().push(x),
                _ => {},
            }
        }
    }

    //println!("\nstack");
    //for stacked_crate in &depot[1] {
    //    println!("{}", stacked_crate.to_string());
    //}

    return Some(depot);
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    let stream = stdin();
    let mut foo = stream.lock();
    let mut depot = parse_crates(&mut foo).unwrap();
    let mut raw_instructions = foo.lines();
    raw_instructions.next();
    raw_instructions.next();

    let move_parser = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in raw_instructions {
        let unline = line.unwrap();
        let m = move_parser.captures(&unline).unwrap();
        let inst = Instruction {
            // I really wanted to pull out these 3 and do this in a comprehension
            // But I'm too stupid I guess to figure that out
            count: m.get(1).unwrap().as_str().parse().unwrap(),
            from:  m.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1,
            to:    m.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1,
        };
        for _ in 0..inst.count {
            let from = depot[inst.from].pop().unwrap();
            depot[inst.to].push(from);
        }
    }
    print_depot_top(&depot);

}
