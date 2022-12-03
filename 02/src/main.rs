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
//
// part 2
// X = Need lose
// Y = Need draw
// Z = Need win
//
// A, Rock,     1pt
// B, Paper,    2pt
// C, Scissors, 3pt
//
// A X => A C => 0 + 3 => 3
// A Y => A A => 3 + 1 => 4
// A Z => A B => 6 + 2 => 8
// B X => B A => 0 + 1 => 1
// B Y => B B => 3 + 2 => 5
// B Z => B C => 6 + 3 => 9
// C X => C B => 0 + 2 => 2
// C Y => C C => 3 + 3 => 6
// C Z => C A => 6 + 1 => 7

use std::collections::HashMap;
//use std::io::prelude::*;
use std::io::{stdin, BufRead};

fn main() {
    let pt_one_codex = HashMap::from([
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
    let pt_two_codex = HashMap::from([
        ("A X", 3),
        ("A Y", 4),
        ("A Z", 8),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 2),
        ("C Y", 6),
        ("C Z", 7),
    ]);

    let mut one_sum = 0;
    let mut two_sum = 0;
    for line in stdin().lock().lines() {
        let index = line.unwrap();
        one_sum += pt_one_codex.get(&*index).unwrap();
        two_sum += pt_two_codex.get(&*index).unwrap();
        //println!("{}", codex.get(&*line.unwrap()).unwrap());
    }
    println!("{one_sum} {two_sum}")
}
