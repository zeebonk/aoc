// --- Part Two ---
//
// Then, you notice the instructions continue on the back of the Recruiting
// Document. Easter Bunny HQ is actually at the first location you visit twice.
//
// For example, if your instructions are R8, R4, R4, R8, the first location you
// visit twice is 4 blocks away, due East.
//
// How many blocks away is the first location you visit twice?

extern crate aoc2016;
extern crate regex;

use aoc2016::day_1::Santa;
use aoc2016::read_from_stdin;
use regex::Regex;
use std::collections::HashSet;

fn main() {
    let mut santa = Santa::new();
    let mut visited_locations = HashSet::new();
    visited_locations.insert((santa.x, santa.y));

    let instructions = read_from_stdin().unwrap();
    let re = Regex::new(r"([RL])(\d+)").unwrap();

    for instruction in re.captures_iter(&instructions) {
        match instruction.at(1) {
            Some("R") => santa.turn_right(),
            Some("L") => santa.turn_left(),
            x => panic!("Unsupported instruction: {:?}", x),
        };

        let step_size: i32 = instruction.at(2).unwrap().parse().unwrap();
        for _ in 0..step_size {
            santa.move_forwards(1);
            // Replace returns already existing values or None
            if visited_locations.replace((santa.x, santa.y)).is_some() {
                println!("{}", santa.x.abs() + santa.y.abs());
                return;
            }
        }
    }
}
