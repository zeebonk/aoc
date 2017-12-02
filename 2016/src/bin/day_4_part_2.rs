/* --- Part Two ---

With all the decoy data out of the way, it's time to decrypt this list and get
moving.

The room names are encrypted by a state-of-the-art shift cipher, which is nearly
unbreakable without the right software. However, the information kiosk designers
at Easter Bunny HQ were not expecting to deal with a master cryptographer like
yourself.

To decrypt a room name, rotate each letter forward through the alphabet a number
of times equal to the room's sector ID. A becomes B, B becomes C, Z becomes A,
and so on. Dashes become spaces.

For example, the real name for qzmt-zixmtkozy-ivhz-343 is very encrypted name.

What is the sector ID of the room where North Pole objects are stored?
*/

extern crate aoc2016;
extern crate regex;

use aoc2016::read_from_stdin;
use regex::Regex;

fn main() {
    let input = read_from_stdin().unwrap();

    let re = Regex::new(r"([a-z-]+)(\d+)\[[a-z]+\]").unwrap();

    for cap in re.captures_iter(&input) {
        let sector_id = cap.at(2).unwrap().parse::<i32>().unwrap();
        let name: String =
            cap
            .at(1)
            .unwrap()
            .chars()
            // Map chars to i32s from a (0) to z (25)
            .map(|c| c as i32 - 97)
            // Shift & wrap
            .map(|c| (c + sector_id) % 26)
            // Map chars back to actual ascii values
            .map(|c| (c  + 97) as u8 as char)
            .collect();

        // Assignment not really clear, assume that 'north' is in the name
        if name.contains("north") {
            println!("{} {}", name, sector_id);
        }
    }
}
