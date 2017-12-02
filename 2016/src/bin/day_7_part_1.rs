/* --- Day 7: Internet Protocol Version 7 ---

While snooping around the local network of EBHQ, you compile a list of IP
addresses (they're IPv7, of course; IPv6 is much too limited). You'd like to
figure out which IPs support TLS (transport-layer snooping).

An IP supports TLS if it has an Autonomous Bridge Bypass Annotation, or ABBA. An
ABBA is any four-character sequence which consists of a pair of two different
characters followed by the reverse of that pair, such as xyyx or abba. However,
the IP also must not have an ABBA within any hypernet sequences, which are
contained by square brackets.

For example:

- abba[mnop]qrst supports TLS (abba outside square brackets).
- abcd[bddb]xyyx does not support TLS (bddb is within square brackets, even
  though xyyx is outside square brackets).
- aaaa[qwer]tyui does not support TLS (aaaa is invalid; the interior characters
  must be different).
- ioxxoj[asdfgh]zxcvbn supports TLS (oxxo is outside square brackets, even
  though it's within a larger string).

How many IPs in your puzzle input support TLS?
*/

use std::io::BufRead;
use std::io;

fn is_abba(buf: &[u8]) -> bool {
    return
        buf[0] == buf[3] &&
        buf[1] == buf[2] &&
        buf[0] != buf[1];
}

fn supports_tls(buf: &Vec<u8>) -> bool{
    let mut supports_tls = false;
    let mut in_hypernet = false;

    for i in 0..buf.len() - 3 {
        in_hypernet = match buf[i + 3] {
            91 => true,  // [
            93 => false,  // ]
            _ => in_hypernet,
        };

        match (is_abba(&buf[i..i + 4]), in_hypernet) {
            (true, true) => return false,
            (true, false) => supports_tls = true,
            _ => {},
        }
    }

    return supports_tls;
}

fn main() {
    let stdin = io::stdin();

    let mut tls_supporting_ips = 0;

    for line in stdin.lock().lines().filter_map(|x| x.ok()) {
        let buf = line.into_bytes();
        if supports_tls(&buf) {
            tls_supporting_ips += 1;
        }
    }

    println!("{}" , tls_supporting_ips);
}
