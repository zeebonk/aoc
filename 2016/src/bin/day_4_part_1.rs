/* --- Day 4: Security Through Obscurity ---

Finally, you come across an information kiosk with a list of rooms. Of course,
the list is encrypted and full of decoy data, but the instructions to decode the
list are barely hidden nearby. Better remove the decoy data first.

Each room consists of an encrypted name (lowercase letters separated by dashes)
followed by a dash, a sector ID, and a checksum in square brackets.

A room is real (not a decoy) if the checksum is the five most common letters in
the encrypted name, in order, with ties broken by alphabetization. For example:

- aaaaa-bbb-z-y-x-123[abxyz] is a real room because the most common letters are
  a (5), b (3), and then a tie between x, y, and z, which are listed
  alphabetically.
- a-b-c-d-e-f-g-h-987[abcde] is a real room because although the letters are all
  tied (1 of each), the first five are listed alphabetically.
- not-a-real-room-404[oarel] is a real room.
- totally-real-room-200[decoy] is not.

Of the real rooms from the list above, the sum of their sector IDs is 1514.

What is the sum of the sector IDs of the real rooms?
*/

extern crate aoc2016;
extern crate itertools;
extern crate regex;

use aoc2016::read_from_stdin;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = read_from_stdin().unwrap();

    let re = Regex::new(r"([a-z-]+)(\d+)\[([a-z]+)\]").unwrap();
    let mut char_counts = HashMap::new();
    let mut sector_sum = 0;

    for cap in re.captures_iter(&input) {
        let sector_id = cap.at(2).unwrap().parse::<i32>().unwrap();
        let checksum = cap.at(3).unwrap();
        let name = cap.at(1).unwrap();

        // Get counts per char, excluding '-'
        for c in name.chars().filter(|x| *x != '-') {
            *(char_counts.entry(c).or_insert(0)) += 1;
        }

        let checksums_match =
            char_counts
            // Clear char_counts while consuming, preparing for next input line
            .drain()
            // Sort by "count * 1000 + c as i32" as "c as i32 < 1000" and thus
            // sorts by both count and alphabet
            .sorted_by(
                |&a, &b| Ord::cmp(
                    &(b.1 * 1000 - b.0 as i32),
                    &(a.1 * 1000 - a.0 as i32)
                )
            )
            .into_iter()
            // Get actual checksum by taking the highest 5 scoring chars
            .take(5)
            .map(|x| x.0)
            // Check that the checksums are equal by zipping and comparing
            .zip(checksum.chars())
            .all(|x| x.0 == x.1);

        if checksums_match {
            sector_sum += sector_id;
        }
    }

    println!("{}", sector_sum);
}
