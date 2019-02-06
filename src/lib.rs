//! # cows
//!
//! Get random ASCII cow
//!

extern crate rand;

use rand::Rng;

mod cows;

pub fn random() -> String {
    let mut rng = rand::thread_rng();
    let (cow, desc) = cows::COWS[rng.gen_range(0, cows::COWS.len())];
    return [cow, desc].join("\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random() {
        assert!(random().len() > 0);
    }
}
