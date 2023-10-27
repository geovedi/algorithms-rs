extern crate rand;

use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::seq::SliceRandom; // Import for shuffling

struct Pearson {
    t: [u8; 256],
}

impl Pearson {
    fn new() -> Pearson {
        let mut t = [0u8; 256];
        for i in 0..256 {
            t[i] = i as u8;
        }

        // Shuffle T using a fixed random seed for reproducibility
        let seed: [u8; 32] = [0; 32];
        // Initialize the random number generator with the fixed seed
        let mut rng: StdRng = SeedableRng::from_seed(seed);

        // Shuffle the array using Fisher-Yates algorithm
        t.shuffle(&mut rng);

        Pearson { t }
    }

    fn hash(&self, input: &str) -> u8 {
        let mut h = 0u8;
        for byte in input.bytes() {
            h = self.t[(h ^ byte) as usize];
        }
        h
    }
}

fn main() {
    let pearson = Pearson::new();
    let test_cases = [
        ("Hello", 248),
        ("World", 158),
        ("Rust!", 124),
    ];

    for (i, (input, expected)) in test_cases.iter().enumerate() {
        let hash = pearson.hash(input);
        let passed = hash == *expected;
        println!("Input: {}", input);
        println!("Hash: {}", hash);
        println!("Test case {}: {}", i + 1, if passed { "PASSED" } else { "FAILED" });
        println!();
    }
}
