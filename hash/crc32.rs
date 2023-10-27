struct CRC32Cipher;

impl CRC32Cipher {
    fn new() -> CRC32Cipher {
        CRC32Cipher
    }

    fn encrypt(&self, input: &str) -> u32 {
        let mut crc = 0xFFFFFFFFu32;
        for byte in input.bytes() {
            crc ^= byte as u32;
            for _ in 0..8 {
                let mask = (crc & 1).wrapping_neg();
                crc = (crc >> 1) ^ (0xEDB88320u32 & mask);
            }
        }
        (!crc) & 0xFFFFFFFFu32
    }
}

fn test_crc32() {
    let cipher = CRC32Cipher::new();
    let test_cases = [
        ("Hello, world!", 0xEBE6C6E6),
        ("The quick brown fox jumps over the lazy dog", 0x414FA339),
        ("", 0x00000000),
    ];

    for (i, (input, expected)) in test_cases.iter().enumerate() {
        let output = cipher.encrypt(input);
        let status = if output == *expected { "PASSED" } else { "FAILED" };
        println!("Test {}: {}", i + 1, status);
        println!("Input:    '{}'", input);
        println!("Expected: '{:08X}'", expected);
        println!("Output:   '{:08X}'", output);
        println!();
    }
}

fn main() {
    test_crc32();
}
