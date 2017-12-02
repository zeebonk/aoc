/* --- Day 3: Part Two ---

Now that you've helpfully marked up their design documents, it occurs to you
that triangles are specified in groups of three vertically. Each set of three
numbers in a column specifies a triangle. Rows are unrelated.

For example, given the following specification, numbers with the same hundreds
digit would be part of the same triangle:

101 301 501
102 302 502
103 303 503
201 401 601
202 402 602
203 403 603

In your puzzle input, and instead reading by columns, how many of the listed
triangles are possible?
*/

extern crate aoc2016;
extern crate itertools;
extern crate regex;

use aoc2016::day_3::valid_triangle;
use aoc2016::read_from_stdin;
use itertools::Itertools;
use regex::Regex;

fn main() {
    let input = read_from_stdin().unwrap();
    let re = Regex::new(r"(\d+)").unwrap();

    let mut count = 0;
    let mut buf = [0; 9];

    for chunk in &re.captures_iter(&input).chunks(9) {
        for (i, m) in chunk.enumerate() {
            buf[i] = m.at(1).unwrap().parse::<i32>().unwrap();
        }

        count += (0..3).filter(|&i| valid_triangle(buf[0 + i], buf[3 + i], buf[6 + i])).count();
    }

    println!("{}", count);
}
