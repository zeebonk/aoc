/* --- Day 6: Signals and Noise ---

Something is jamming your communications with Santa. Fortunately, your signal is
only partially jammed, and protocol in situations like this is to switch to a
simple repetition code to get the message through.

In this model, the same message is sent repeatedly. You've recorded the
repeating message signal (your puzzle input), but the data seems quite
corrupted - almost too badly to recover. Almost.

All you need to do is figure out which character is most frequent for each
position. For example, suppose you had recorded the following messages:

eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar

The most common character in the first column is e; in the second, a; in the
third, s, and so on. Combining these characters returns the error-corrected
message, easter.

Given the recording in your puzzle input, what is the error-corrected version of
the message being sent?
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
        let mut max_cnt = 0;
        let mut max_chr: u8 = 0;
        for j in 0..26 {
            if counts[i][j] > max_cnt {
                max_cnt = counts[i][j];
                max_chr = j as u8;
            }
        }

        print!("{}", (max_chr + 97) as char);
    }

    println!("");
}
