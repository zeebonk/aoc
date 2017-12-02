/* --- Day 10: Balance Bots ---

You come upon a factory in which many robots are zooming around handing small
microchips to each other.

Upon closer examination, you notice that each bot only proceeds when it has two
microchips, and once it does, it gives each one to a different bot or puts it in
a marked "output" bin. Sometimes, bots take microchips from "input" bins, too.

Inspecting one of the microchips, it seems like they each contain a single
number; the bots must use some logic to decide what to do with each chip. You
access the local control computer and download the bots' instructions (your
puzzle input).

Some of the instructions specify that a specific-valued microchip should be
given to a specific bot; the rest of the instructions indicate what a given bot
should do with its lower-value or higher-value chip.

For example, consider the following instructions:

  value 5 goes to bot 2
  bot 2 gives low to bot 1 and high to bot 0
  value 3 goes to bot 1
  bot 1 gives low to output 1 and high to bot 0
  bot 0 gives low to output 2 and high to output 0
  value 2 goes to bot 2

- Initially, bot 1 starts with a value-3 chip, and bot 2 starts with a value-2
  chip and a value-5 chip.
- Because bot 2 has two microchips, it gives its lower one (2) to bot 1 and its
  higher one (5) to bot 0.
- Then, bot 1 has two microchips; it puts the value-2 chip in output 1 and gives
  the value-3 chip to bot 0.
- Finally, bot 0 has two microchips; it puts the 3 in output 2 and the 5 in
  output 0.
- In the end, output bin 0 contains a value-5 microchip, output bin 1 contains a
  value-2 microchip, and output bin 2 contains a value-3 microchip. In this
  configuration, bot number 2 is responsible for comparing value-5 microchips
  with value-2 microchips.

Based on your instructions, what is the number of the bot that is responsible
for comparing value-61 microchips with value-17 microchips?
*/

use std::io::BufRead;
use std::cmp;
use std::io;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Source {
    Some(i32),
    Low(usize),
    High(usize),
    None,
}

impl Source {
    from_str(string: &str, index: usize) -> Source {
        match string {
        "low" => Source::Low(index),
        "high" => Source::High(index),
        _ => Source::None,
    }
}

#[derive(Copy, Clone, Debug)]
struct Bot {
    in1: Source,
    in2: Source,
}

fn main() {
    let mut bots = [Bot {in1: Source::None, in2: Source::None}; 300];
    let mut outputs = [Source::None; 300];
    let mut cache: HashMap<Source, i32> = HashMap::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines().filter_map(|x| x.ok()) {
        println!("{}", line);

        let parts: Vec<&str>= line.split(' ').collect();

        if parts[0] == "value" {
            let value: i32 = parts[1].parse().unwrap();
            let index: usize = parts[5].parse().unwrap();
            bots[index].give(Source::Some(value));
            continue;

        }

        let source_index: usize = parts[1].parse().unwrap();
        let target1_index: usize = parts[6].parse().unwrap();
        let target2_index: usize = parts[11].parse().unwrap();

        match (parts[5], Source::from_str(parts[3], source_index)) {
            ("bot", s) => bots[target1_index].give(s),
            ("output", s) => outputs[target1_index] = s,
            _ => panic!("Unsupported target"),
        }

        match (parts[10], Source::from_str(parts[8], source_index)) {
            ("bot", s) => bots[target2_index].give(s),
            ("output", s) => outputs[target2_index] = s,
            _ => panic!("Unsupported target"),
        }
    }

    for i in 0..21 {
        let value = match outputs[i] {
            Source::Some(v) => v,
            Source::High(i) => bots[i].high(&bots, &mut cache),
            Source::Low(i) => bots[i].low(&bots, &mut cache),
            Source::None => panic!("Can't evaluate source 'None'"),
        };

        println!("{:?}", value);
    }
}
