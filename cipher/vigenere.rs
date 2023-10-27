struct VigenereCipher {
    key: String,
}

impl VigenereCipher {
    fn new(key: &str) -> VigenereCipher {
        VigenereCipher {
            key: key.to_uppercase(),
        }
    }

    fn encrypt(&self, text: &str) -> String {
        let mut encrypted = String::new();
        let mut key_index = 0;
        for c in text.chars() {
            if c.is_ascii_uppercase() {
                let offset = self.key.chars().nth(key_index).unwrap() as u8 - b'A';
                encrypted.push((((c as u8 - b'A' + offset) % 26) + b'A') as char);
                key_index = (key_index + 1) % self.key.len();
            } else {
                encrypted.push(c);
            }
        }
        encrypted
    }

    fn decrypt(&self, text: &str) -> String {
        let mut decrypted = String::new();
        let mut key_index = 0;
        for c in text.chars() {
            if c.is_ascii_uppercase() {
                let offset = self.key.chars().nth(key_index).unwrap() as u8 - b'A';
                decrypted.push((((c as u8 - b'A' + 26 - offset) % 26) + b'A') as char);
                key_index = (key_index + 1) % self.key.len();
            } else {
                decrypted.push(c);
            }
        }
        decrypted
    }
}

fn test_vigenere_cipher() {
    let cipher = VigenereCipher::new("KEY");
    let text = "HELLO WORLD";
    let encrypted = cipher.encrypt(text);
    let decrypted = cipher.decrypt(&encrypted);

    assert_ne!(encrypted, text, "Test 1 FAILED");
    assert_eq!(decrypted, text, "Test 2 FAILED");
    assert_eq!(cipher.encrypt("ABC"), cipher.encrypt("ABC"), "Test 3 FAILED");

    println!("All tests PASSED");
}

fn main() {
    test_vigenere_cipher();
}
