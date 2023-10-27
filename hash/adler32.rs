struct Adler32Cipher;

impl Adler32Cipher {
    fn new() -> Adler32Cipher {
        Adler32Cipher
    }

    fn encrypt(&self, input: &str) -> u32 {
        const MOD_ADLER: u32 = 65521;
        let mut a: u32 = 1;
        let mut b: u32 = 0;

        for byte in input.bytes() {
            a = (a + byte as u32) % MOD_ADLER;
            b = (b + a) % MOD_ADLER;
        }

        (b << 16) | a
    }
}

fn test_adler32_cipher() {
    let cipher = Adler32Cipher::new();

    let test_cases = [
        ("Wikipedia", 0x11E60398),
        ("Hello, world!", 0x205E048A),
        ("", 0x1),
    ];

    for (i, test_case) in test_cases.iter().enumerate() {
        let output = cipher.encrypt(test_case.0);
        let status = if output == test_case.1 { "PASSED" } else { "FAILED" };
        println!(
            "Test {}: input='{}', expected='{:#010X}', output='{:#010X}' - {}",
            i + 1,
            test_case.0,
            test_case.1,
            output,
            status
        );
    }
}

fn main() {
    test_adler32_cipher();
}
