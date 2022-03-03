use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

/// Generate a random string with a given length `n`
pub fn random_string(n: usize) -> String {
    thread_rng()
		.sample_iter(&Alphanumeric)
        .take(n)
		.map(char::from)
        .collect()
}
