struct AtbashCipher;

impl AtbashCipher {
    fn new() -> AtbashCipher {
        AtbashCipher
    }

    fn encrypt(&self, plaintext: &str) -> String {
        let mut ciphertext = String::new();
        for c in plaintext.chars() {
            let byte = c as u8;
            if byte >= b'A' && byte <= b'Z' {
                ciphertext.push(char::from(b'Z' - (byte - b'A')));
            } else if byte >= b'a' && byte <= b'z' {
                ciphertext.push(char::from(b'z' - (byte - b'a')));
            } else {
                ciphertext.push(c);
            }
        }
        ciphertext
    }
}

fn test_atbash_cipher() {
    let cipher = AtbashCipher::new();
    let test_cases = vec![
        ("ABCDEFGHIJKLMNOPQRSTUVWXYZ", "ZYXWVUTSRQPONMLKJIHGFEDCBA"),
        ("abcdefghijklmnopqrstuvwxyz", "zyxwvutsrqponmlkjihgfedcba"),
        ("1234567890", "1234567890"),
        ("HELLO WORLD", "SVOOL DLIOW"),
        ("Atbash Cipher", "Zgyzhs Xrksvi"),
    ];

    for (i, (plaintext, expected_ciphertext)) in test_cases.iter().enumerate() {
        let actual_ciphertext = cipher.encrypt(plaintext);
        let status = if expected_ciphertext == &actual_ciphertext {
            "PASSED"
        } else {
            "FAILED"
        };
        println!("Test {}: {}", i + 1, status);
    }
}

fn main() {
    test_atbash_cipher();
}
