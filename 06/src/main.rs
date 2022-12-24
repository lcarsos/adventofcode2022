use std::io::{stdin, BufRead};
use std::collections::HashSet;

fn signal_lock_on(stream: &Vec<u8>) -> Option<usize> {
    for sop in 3..stream.len() {
        // https://stackoverflow.com/questions/39803237/build-hashset-from-a-vector-in-rust
        // works on the whole array, but I just want a slice
        let sequence = stream.get(sop-3..sop).unwrap();
        //println!("test: {}", sequence.to_string());
        let mut test = HashSet::<u8>::from_iter(sequence.cloned());
        // error[E0599]: the method `cloned` exists for reference `&[u8]`, but its trait bounds were not satisfied
        //   --> src/main.rs:10:58
        //    |
        // 10 |         let mut test = HashSet::<u8>::from_iter(sequence.cloned());
        //    |                                                          ^^^^^^ method cannot be called on `&[u8]` due to unsatisfied trait bounds
        //    |
        //    = note: the following trait bounds were not satisfied:
        //            `&[u8]: Iterator`
        //            which is required by `&mut &[u8]: Iterator`
        //            `[u8]: Iterator`
        //            which is required by `&mut [u8]: Iterator`
        if test.len() == 4 {
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
    let mut signal: Vec<u8> = Vec::from(in_string);
    let target = signal_lock_on(&signal).unwrap();
}
