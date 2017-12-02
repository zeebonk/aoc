/* --- Day 8: Two-Factor Authentication ---

You come across a door implementing what you can only assume is an
implementation of two-factor authentication after a long game of requirements
telephone.

To get past the door, you first swipe a keycard (no problem; there was one on a
nearby desk). Then, it displays a code on a little screen, and you type that
code on a keypad. Then, presumably, the door unlocks.

Unfortunately, the screen has been smashed. After a few minutes, you've taken
everything apart and figured out how it works. Now you just have to work out
what the screen would have displayed.

The magnetic strip on the card you swiped encodes a series of instructions for
the screen; these instructions are your puzzle input. The screen is 50 pixels
wide and 6 pixels tall, all of which start off, and is capable of three somewhat
peculiar operations:

- rect AxB turns on all of the pixels in a rectangle at the top-left of the
  screen which is A wide and B tall.
- rotate row y=A by B shifts all of the pixels in row A (0 is the top row) right
  by B pixels. Pixels that would fall off the right end appear at the left end
  of the row.
- rotate column x=A by B shifts all of the pixels in column A (0 is the left
  column) down by B pixels. Pixels that would fall off the bottom appear at the
  top of the column.

For example, here is a simple sequence on a smaller screen:

  rect 3x2 creates a small rectangle in the top-left corner:

  ###....
  ###....
  .......

  rotate column x=1 by 1 rotates the second column down by one pixel:

  #.#....
  ###....
  .#.....

  rotate row y=0 by 4 rotates the top row right by four pixels:

  ....#.#
  ###....
  .#.....

  rotate column x=1 by 1 again rotates the second column down by one pixel,
  causing the bottom pixel to wrap back to the top:

  .#..#.#
  #.#....
  .#.....

As you can see, this display technology is extremely powerful, and will soon
dominate the tiny-code-displaying-screen market. That's what the advertisement
on the back of the display tries to convince you, anyway.

There seems to be an intermediate check of the voltage used by the display:
after you swipe your card, if the screen did work, how many pixels should be
lit?
*/

extern crate regex;
#[macro_use] extern crate itertools;

use regex::Regex;
use std::io::{self, BufRead};

struct Grid {
    width: usize,
    height: usize,
    fields: [[bool; 50]; 6],
    buffer: [bool; 50],
}

impl Grid {
    fn new() -> Grid {
        Grid {
            width: 50,
            height: 6,
            fields: [[false; 50]; 6],
            buffer: [false; 50],
        }
    }

    fn rectangle(&mut self, wide: usize, tall: usize) {
        for (y, x) in iproduct!(0..tall, 0..wide) {
            self.fields[y][x] = true;
        }
    }

    fn rotate_row(&mut self, y: usize, pixels: usize) {
        let row = &mut self.fields[y];
        let offset = self.width - pixels;

        for x in 0..self.width {
            self.buffer[x] = row[(x + offset) % self.width];
        }

        for x in 0..self.width {
            row[x] = self.buffer[x];
        }
    }

    fn rotate_column(&mut self, x: usize, pixels: usize) {
        let offset = self.height - pixels;

        for y in 0..self.height {
            self.buffer[y] = self.fields[(y + offset) % self.height][x];
        }

        for y in 0..self.height {
            self.fields[y][x] = self.buffer[y];
        }
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", if self.fields[y][x] {'#'} else {'.'});
            }
            println!("");
        }
        println!("");
    }

    fn count(&self) -> usize {
        return
            iproduct!(0..self.height, 0..self.width)
            .fold(0, |acc, (y, x)| acc + if self.fields[y][x] {1} else {0});
    }
}

fn main() {
    let mut grid = Grid::new();

    let re = Regex::new(r"(rect|column|row)\D+(\d+)\D+(\d+)").unwrap();
    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|x| x.unwrap()) {
        let capture = re.captures(&line).unwrap();

        let op = capture.at(1).unwrap().as_bytes()[2] as char;
        let v1: usize = capture.at(2).unwrap().parse().unwrap();
        let v2: usize = capture.at(3).unwrap().parse().unwrap();

        match op {
            'c' => grid.rectangle(v1, v2),
            'w' => grid.rotate_row(v1, v2),
            'l' => grid.rotate_column(v1, v2),
            _ => {},
        }
    }

    grid.print();
    println!("\nCount: {}", grid.count());
}
