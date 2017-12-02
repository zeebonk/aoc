/* --- Part Two ---

Of course, that would be the message - if you hadn't agreed to use a modified
repetition code instead.

In this modified code, the sender instead transmits what looks like random data,
but for each character, the character they actually want to send is slightly
less likely than the others. Even after signal-jamming noise, you can look at
the letter distributions in each column and choose the least common letter to
reconstruct the original message.

In the above example, the least common character in the first column is a; in
the second, d, and so on. Repeating this process for the remaining characters
produces the original message, advent.

Given the recording in your puzzle input and this new decoding methodology, what
is the original message that Santa is trying to send?
*/

use std::io::BufRead;
use std::io;

fn main() {
    let mut counts = [[0; 26]; 8];

    let stdin = io::stdin();
    for line in stdin.lock().lines().filter_map(|x| x.ok()) {
        for (i, c) in line.chars().enumerate() {
            counts[i][c as usize - 97] += 1;
        }
    }

    for i in 0..8 {
        let mut min_cnt = 10000;
        let mut min_chr: u8 = 0;
        for j in 0..26 {
            if counts[i][j] < min_cnt {
                min_cnt = counts[i][j];
                min_chr = j as u8;
            }
        }

        print!("{}", (min_chr + 97) as char);
    }

    println!("");
}
