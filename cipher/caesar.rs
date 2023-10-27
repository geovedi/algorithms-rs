struct CaesarCipher {
    key: i32,
    alphabet_size: u8,
}

impl CaesarCipher {
    fn new(key: i32) -> CaesarCipher {
        CaesarCipher {
            key,
            alphabet_size: 26,
        }
    }

    fn encrypt(&self, text: &str) -> String {
        self.process(text, self.key)
    }

    fn decrypt(&self, text: &str) -> String {
        self.process(text, -self.key)
    }

    fn process(&self, text: &str, key: i32) -> String {
        let mut result = String::new();
        for c in text.chars() {
            let is_upper_case = c.is_ascii_uppercase();
            let is_lower_case = c.is_ascii_lowercase();

            if is_upper_case || is_lower_case {
                let base = if is_upper_case { b'A' } else { b'a' };
                let index = ((c as u8 - base) as i32 + key + self.alphabet_size as i32) % self.alphabet_size as i32;
                let processed_char = (base + index as u8) as char;
                result.push(processed_char);
            } else {
                result.push(c);
            }
        }
        result
    }
}

fn test_caesar() {
    let test_cases = vec![
        (1, "Hello World!", "Ifmmp Xpsme!"),
        (3, "ABCDEFGHIJKLMNOPQRSTUVWXYZ", "DEFGHIJKLMNOPQRSTUVWXYZABC"),
        (5, "The Quick Brown Fox", "Ymj Vznhp Gwtbs Ktc"),
        (13, "Caesar Cipher", "Pnrfne Pvcure"),
    ];

    for (i, (key, text, encrypted)) in test_cases.iter().enumerate() {
        let caesar_cipher = CaesarCipher::new(*key);
        let encrypted_text = caesar_cipher.encrypt(text);
        let decrypted_text = caesar_cipher.decrypt(&encrypted_text);

        println!("Test Case {}:", i + 1);
        println!("  Key: {}", key);
        println!("  Original Text: {}", text);
        println!("  Encrypted Text: {}", encrypted_text);
        println!("  Decrypted Text: {}", decrypted_text);

        let encryption_success = encrypted_text == *encrypted;
        let decryption_success = decrypted_text == *text;

        if encryption_success && decryption_success {
            println!("  Result: PASSED\n");
        } else {
            println!("  Result: FAILED\n");
        }
    }
}

fn main() {
    test_caesar();
}
