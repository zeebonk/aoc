/* --- Part Two ---

Apparently, the file actually uses version two of the format.

In version two, the only difference is that markers within decompressed data are
decompressed. This, the documentation explains, provides much more substantial
compression capabilities, allowing many-gigabyte files to be stored in only a
few kilobytes.

For example:
- (3x3)XYZ still becomes XYZXYZXYZ, as the decompressed section contains no
  markers.
- X(8x2)(3x3)ABCY becomes XABCABCABCABCABCABCY, because the decompressed data
  from the (8x2) marker is then further decompressed, thus triggering the (3x3)
  marker twice for a total of six ABC sequences.
- (27x12)(20x12)(13x14)(7x10)(1x12)A decompresses into a string of A repeated
  241920 times.
- (25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN becomes 445
  characters long.

Unfortunately, the computer you brought probably doesn't have enough memory to
actually decompress the file; you'll have to come up with another way to get its
decompressed length.

What is the decompressed length of the file using this improved format?
*/

extern crate aoc2016;

use aoc2016::read_from_stdin;

fn decomp_length(slice: &str) -> usize {
    let result = slice.find('(');
    if result.is_none() {
        return slice.len();
    }

    let open_pos = result.unwrap();
    let cross_pos = slice.find('x').unwrap();
    let close_pos = slice.find(')').unwrap();

    let width: usize = slice[open_pos + 1..cross_pos].parse().unwrap();
    let repeats: usize = slice[cross_pos + 1..close_pos].parse().unwrap();

    let repeated = &slice[close_pos + 1..close_pos + 1 + width];
    let remaining = &slice[close_pos + 1 + width..];

    open_pos + repeats * decomp_length(repeated) + decomp_length(remaining)
}

fn main() {
    let input = read_from_stdin().expect("Failed to read from stdin");
    println!("{}", decomp_length(input.trim_right()));
}
