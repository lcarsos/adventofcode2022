use std::str;
use std::io::{stdin, BufRead};
use std::collections::HashSet;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn signal_lock_on(stream: &Vec<u8>, width: usize) -> Option<usize> {
    for sop in width..stream.len() {
        // https://stackoverflow.com/questions/39803237/build-hashset-from-a-vector-in-rust
        // works on the whole array, but I just want a slice
        // sidenote: apparently stream[sop-3..sop] size is not known at compile time
        let sequence = stream.get(sop-width..sop).unwrap().iter().cloned();
        {
            //let debug_str = sequence.clone().collect::<Vec<u8>>();
            //println!("test: {}", str::from_utf8(&sequence.clone().collect::<Vec<u8>>()).unwrap());
        }
        let mut test = HashSet::<u8>::from_iter(sequence);
        if test.len() == width {
            return Some(sop);
        }
    }
    return None;
}

fn main() {
    let mut in_string = String::new();
    match stdin().read_line(&mut in_string) {
        Ok(_) => {},
        Err(e) => panic!("panic: {e}"),
    }
    let signal: Vec<u8> = Vec::from(in_string);
    let target = signal_lock_on(&signal, 4).unwrap();
    let message = signal_lock_on(&signal, 14).unwrap();
    println!("{target}");
    println!("{message}");
}
