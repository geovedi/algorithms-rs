struct DJB2Cipher;

impl DJB2Cipher {
    fn new() -> DJB2Cipher {
        DJB2Cipher
    }

    fn encrypt(&self, input: &str) -> u32 {
        let mut hash = 5381u32;
        for byte in input.bytes() {
            hash = (hash.wrapping_mul(33)).wrapping_add(byte as u32);
        }
        hash & 0xFFFFFFFFu32
    }
}

fn test_djb2() {
    let cipher = DJB2Cipher::new();
    let test_cases = [
        ("Hello, world!", 0xE18796AE),
        ("The quick brown fox jumps over the lazy dog", 0x34CC38DE),
        ("", 0x1505),
    ];

    for (i, (input, expected)) in test_cases.iter().enumerate() {
        let output = cipher.encrypt(input);
        let status = if output == *expected { "PASSED" } else { "FAILED" };
        println!("Test {}: {}", i + 1, status);
        println!("Input:    '{}'", input);
        println!("Expected: '{:X}'", expected);
        println!("Output:   '{:X}'", output);
        println!();
    }
}

fn main() {
    test_djb2();
}
