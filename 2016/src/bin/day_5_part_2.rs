/* --- Part Two ---

As the door slides open, you are presented with a second door that uses a
slightly more inspired security mechanism. Clearly unimpressed by the last
version (in what movie is the password decrypted in order?!), the Easter Bunny
engineers have worked out a better solution.

Instead of simply filling in the password from left to right, the hash now also
indicates the position within the password to fill. You still look for hashes
that begin with five zeroes; however, now, the sixth character represents the
position (0-7), and the seventh character is the character to put in that
position.

A hash result of 000001f means that f is the second character in the password.
Use only the first result for each position, and ignore invalid positions.

For example, if the Door ID is abc:

- The first interesting hash is from abc3231929, which produces 0000015...; so,
  5 goes in position 1: _5______.
- In the previous method, 5017308 produced an interesting hash; however, it is
  ignored, because it specifies an invalid position (8).
- The second interesting hash is at index 5357525, which produces 000004e...;
  so, e goes in position 4: _5__e___.

You almost choke on your popcorn as the final character falls into place,
producing the password 05ace8e3.

Given the actual Door ID and this new method, what is the password? Be extra
proud of your solution if it uses a cinematic "decrypting" animation.
*/

extern crate aoc2016;
extern crate crypto;

use aoc2016::day_5::{has_5_leading_zeros_in_hex, byte_to_hex_char};
use crypto::digest::Digest;
use crypto::md5::Md5;
use std::fmt::Write;

fn main() {
    let mut i = 0;
    let mut md5 = Md5::new();
    let mut hash_buffer = [0 as u8; 16];
    let mut key_buffer = String::with_capacity(64);
    let mut password = ['_'; 8];
    let mut found_characters = 0;

    while found_characters < 8 {
        let _ = write!(&mut key_buffer, "wtnhxymk{}", i);
        md5.input_str(&key_buffer);
        md5.result(&mut hash_buffer);

        if has_5_leading_zeros_in_hex(&hash_buffer) {
            let index = (hash_buffer[2] & 0b0000_1111) as usize;
            if index < 8 && password[index] == '_' {
                password[index] = byte_to_hex_char(hash_buffer[3] >> 4);
                found_characters += 1;
            }
        }

        key_buffer.clear();
        md5.reset();
        i += 1;
    }

    println!("{}", password.iter().cloned().collect::<String>());
}
