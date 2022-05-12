use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::iter;

/// Generate a string from a char repeated count times
pub fn repeat_char(c: char, count: usize) -> String {
    iter::repeat(c).take(count).collect::<String>()
}

/// Generate a random string with a given length `n`
pub fn random_string(n: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(n)
        .map(char::from)
        .collect()
}

// The position of zero in the ASCII table
const ASCII_ZERO: u8 = 48;

// TODO: is this over-engineering?
/// Convert a `Vec<u8>` into a `u32` assuming it was first read as a num literal.
///
/// Like [std::parse](https://doc.rust-lang.org/std/primitive.str.html#method.parse)
/// but for u8 slices
///
/// For example:
/// "49" it was parsed into a vec![52, 57], we expect the u32 value of 49.
/// we do this by adding 40 + 9 i.e ones tens hundreds etc
pub fn u8_slice_to_u32(v: &[u8]) -> u32 {
    v.iter().rev().enumerate().fold(0, |acc, (idx, v)| {
        acc + (*v - ASCII_ZERO) as u32 * 10_u32.pow(idx as u32)
    })
}

// rename to ASCII
fn unsigned_literal_to_u8<T: num::Unsigned + num::ToPrimitive>(v: T) -> u8 {
    v.to_u8().unwrap() + ASCII_ZERO
}

// is this a good idea?
fn unsigned_num_to_ascii<T: num::Unsigned + std::fmt::Display>(n: T) -> Vec<u8> {
    n.to_string().as_bytes().to_vec()
}

// Compare current to next and accumulate counts
// O(n)
// RLE for short
pub fn run_length_encode(cigar: &[u8]) -> Vec<u8> {
    match cigar {
        [] => Vec::<u8>::new(),
        [c] => Vec::from([unsigned_literal_to_u8(1 as u8), *c]),
        [start_char, the_rest @ ..] => {
            // more than one value
            let mut rle = Vec::<u8>::new();
            let mut current: u8 = *start_char;
            let mut count: u32 = 1;

            let mut update_rle = |count: u32, c: u8| {
                rle.extend_from_slice(&unsigned_num_to_ascii(count));
                rle.extend_from_slice(&[c]);
            };

            for c in the_rest {
                if current == *c {
                    count += 1;
                } else {
                    update_rle(count, current);

                    // reset
                    current = *c;
                    count = 1;
                }
            }

            // last run
            update_rle(count, current);

            rle
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeat_char() {
        let count = 10;
        let c = 'X';
        let s = repeat_char(c, count);
        assert_eq!(s.len(), count);
        assert!(s.chars().all(|x| x == c));
    }

    #[test]
    fn test_random_string() {
        let count = 10;
        let s = random_string(count);
        assert_eq!(s.len(), count);
    }
}
