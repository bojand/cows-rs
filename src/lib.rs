//! # cows
//!
//! Get random ASCII cow
//!

extern crate rand;

use rand::Rng;

mod cows;

pub fn random() -> String {
    let mut rng = rand::thread_rng();
    let cow = cows::COWS[rng.gen_range(0, cows::COWS.len())];
    return String::from(cow);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random() {
        assert!(random().len() > 0);
    }
}
