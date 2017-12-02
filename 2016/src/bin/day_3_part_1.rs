// --- Day 3: Squares With Three Sides ---
//
// Now that you can think clearly, you move deeper into the labyrinth of
// hallways and office furniture that makes up this part of Easter Bunny HQ.
// This must be a graphic design department; the walls are covered in
// specifications for triangles.
//
// Or are they?
//
// The design document gives the side lengths of each triangle it describes,
// but... 5 10 25? Some of these aren't triangles. You can't help but mark the
// impossible ones.
//
// In a valid triangle, the sum of any two sides must be larger than the
// remaining side. For example, the "triangle" given above is impossible,
// because 5 + 10 is not larger than 25.
//
// In your puzzle input, how many of the listed triangles are possible?

extern crate aoc2016;
extern crate itertools;
extern crate regex;

use aoc2016::day_3::valid_triangle;
use regex::Regex;
use std::io::{self, BufRead};

fn main() {
    let re = Regex::new(r"(\d+)\D+(\d+)\D+(\d+)").unwrap();
    let mut count = 0;
    let mut buf = [0; 3];

    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|x| x.unwrap()) {
        for (i, val) in re.captures(&line).unwrap().iter().skip(1).enumerate() {
            buf[i] = val.unwrap().parse().unwrap();
        }

        if valid_triangle(buf[0], buf[1], buf[2]) {
            count += 1;
        }
    }

    println!("{}", count);
}
