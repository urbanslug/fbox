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
