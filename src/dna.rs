/// A <=> T
/// C <=> G

/// A 0011
/// T 1100
/// G 0010
/// C 0100

/// base  internal  ASCII
///  A       3         65
///  T      12         84
///  C       2         67
///  G       4         71

fn complement(mut base: u8) -> u8 {
    match base {
        3 => base <<= 2,  // A -> T
        12 => base >>= 2, // T -> A
        2 => base <<= 1,  // C -> G
        4 => base >>= 1,  // G -> C
        0 => base = 0,    // TODO handle N
        _ => {
            panic!("{}", base);
        }
    };
    base
}

fn to_ascii(base: u8) -> u8 {
    let mut lookup: [u8; 13] = [0; 13];
    lookup[3] = 65;  // A
    lookup[12] = 84; // T
    lookup[2] = 67;  // G
    lookup[4] = 71;  // C

    lookup[0] = 78;  // N

    lookup[base as usize]
}

fn from_ascii(base: u8) -> u8 {
    let mut lookup: [u8; 85] = [0; 85];
    lookup[65] = 3; // A
    lookup[67] = 2; // C
    lookup[71] = 4; // G
    lookup[84] = 12; // T

    lookup[78] = 0; // N

    lookup[base as usize]
}

fn complement_ascii(base: u8) -> u8 {
    to_ascii(complement(from_ascii(base)))
}

fn reverse_complement_dna(dna: &[u8]) -> Vec<u8> {
    dna.iter().rev().map(|b| complement_ascii(*b)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rev_comp() {
        let dna = "ACGTAC";

        let rev_cmp = reverse_complement_dna(dna.as_bytes());
        assert_eq!(rev_cmp, "GTACGT".as_bytes());
    }
}
