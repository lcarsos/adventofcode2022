// Opponent
// A = Rock
// B = Paper
// C = Scissors
//
// Me
// X = Rock (1 pt)
// Y = Paper (2 pt)
// Z = Scissors (3 pt)
//
// Win = 6 pts
// Draw = 3 pts
// Loss = 0 pts
//
// X beats C
// Y beats A
// Z beats B
//
// X draws A
// Y draws B
// Z draws C
//
// X loses B
// Y loses C
// Z loses A
//
//       A | B | C
// ----+---+---+---
//   X | 4 | 1 | 7
//   Y | 8 | 5 | 2
//   Z | 3 | 9 | 6
//
// So how much do I over-engineer the solution if it's that easy?

use std::collections::HashMap;
//use std::io::prelude::*;
use std::io::{stdin, BufRead};

fn main() {
    let codex = HashMap::from([
        ("A X", 4),
        ("A Y", 8),
        ("A Z", 3),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 7),
        ("C Y", 2),
        ("C Z", 6),
    ]);

    let mut sum = 0;
    for line in stdin().lock().lines() {
        //let index = line.unwrap();
        sum += codex.get(&*line.unwrap()).unwrap();
        //println!("{}", codex.get(&*line.unwrap()).unwrap());
    }
    println!("{sum}")
}
