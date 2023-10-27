struct FNV1a {
    prime: u32,
    hash_value: u32,
}

impl FNV1a {
    fn new() -> FNV1a {
        FNV1a {
            prime: 0x01000193,
            hash_value: 0x811c9dc5,
        }
    }

    fn hash(&self, input: &str) -> u32 {
        let mut hash = self.hash_value;
        for byte in input.bytes() {
            hash ^= byte as u32;
            hash = hash.wrapping_mul(self.prime);
        }
        hash & 0xFFFFFFFFu32
    }
}

fn test_fnv1a() {
    let fnv1a = FNV1a::new();
    let test_cases = [
        ("Hello", 0xf55c314b),
        ("World", 0xdd60ed33),
        ("Lua!", 0xd41a8e0),
    ];

    for (i, (input, expected)) in test_cases.iter().enumerate() {
        let hash = fnv1a.hash(input);
        let passed = hash == *expected;
        println!("Test case {}: {}", i + 1, if passed { "PASSED" } else { "FAILED" });
    }
}

fn main() {
    test_fnv1a();
}
