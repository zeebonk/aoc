/* --- Part Two ---

You would also like to know which IPs support SSL (super-secret listening).

An IP supports SSL if it has an Area-Broadcast Accessor, or ABA, anywhere in the
supernet sequences (outside any square bracketed sections), and a corresponding
Byte Allocation Block, or BAB, anywhere in the hypernet sequences. An ABA is any
three-character sequence which consists of the same character twice with a
different character between them, such as xyx or aba. A corresponding BAB is the
same characters but in reversed positions: yxy and bab, respectively.

For example:

- aba[bab]xyz supports SSL (aba outside square brackets with corresponding bab
  within square brackets).
- xyx[xyx]xyx does not support SSL (xyx, but no corresponding yxy).
- aaa[kek]eke supports SSL (eke in supernet with corresponding kek in hypernet;
  the aaa sequence is not related, because the interior character must be
  different).
- zazbz[bzb]cdb supports SSL (zaz has no corresponding aza, but zbz has a
  corresponding bzb, even though zaz and zbz overlap).

How many IPs in your puzzle input support SSL?
*/
extern crate itertools;

use std::io::BufRead;
use std::io;
use std::collections::HashSet;
use itertools::Itertools;


fn main() {
    let stdin = io::stdin();

    let mut tls_supporting_ips = 0;
    let mut counts: HashSet<(char, char, bool)> = HashSet::with_capacity(24 * 24 * 2);

    for line in stdin.lock().lines().filter_map(|x| x.ok()) {
        let mut hypernet = false;

        for chunk in &line.chars().chunks(3) {
            match chunk {
                ('[', _, _) => hypernet = true,  // [
                (']', _, _) => hypernet = false,  // ]
                (a, b, c) if a == c && a != b => {
                     if counts.contains(&(b, a, !hypernet)) {
                        tls_supporting_ips += 1;
                        break;
                    }
                    else {
                        counts.insert((a, b, hypernet));
                    }
                },
                _ => {},
            }
        }

        counts.clear();
    }

    println!("{}" , tls_supporting_ips);
}
