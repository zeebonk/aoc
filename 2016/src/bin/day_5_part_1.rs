/* --- Day 5: How About a Nice Game of Chess? ---

You are faced with a security door designed by Easter Bunny engineers that seem
to have acquired most of their security knowledge by watching hacking movies.

The eight-character password for the door is generated one character at a time
by finding the MD5 hash of some Door ID (your puzzle input) and an increasing
integer index (starting with 0).

A hash indicates the next character in the password if its hexadecimal
representation starts with five zeroes. If it does, the sixth character in the
hash is the next character of the password.

For example, if the Door ID is abc:

- The first index which produces a hash that starts with five zeroes is 3231929,
  which we find by hashing abc3231929; the sixth character of the hash, and thus
  the first character of the password, is 1.
- 5017308 produces the next interesting hash, which starts with 000008f82..., so
  the second character of the password is 8.
- The third time a hash starts with five zeroes is for abc5278568, discovering
  the character f.

In this example, after continuing this search a total of eight times, the
password is 18f47a30.

Given the actual Door ID, what is the password?
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
    let mut password = String::with_capacity(8);
    let mut key_buffer = String::with_capacity(64);

    while password.len() < 8 {
        let _ = write!(&mut key_buffer, "wtnhxymk{}", i);
        md5.input_str(&key_buffer);
        md5.result(&mut hash_buffer);

        if has_5_leading_zeros_in_hex(&hash_buffer) {
            password.push(byte_to_hex_char(hash_buffer[2]));
        }

        key_buffer.clear();
        md5.reset();
        i += 1;
    }

    println!("{}", password);
}
