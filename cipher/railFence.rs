struct RailFenceCipher {
    key: usize,
}

impl RailFenceCipher {
    fn new(key: usize) -> RailFenceCipher {
        RailFenceCipher { key }
    }

    fn create_rail_matrix(&self, text: &str) -> Vec<Vec<Option<char>>> {
        let mut rail = vec![vec![None; text.len()]; self.key];

        let mut dir_down = false;
        let mut row = 0;
        let mut col = 0;

        for c in text.chars() {
            rail[row][col] = Some(c);

            if row == 0 || row == self.key - 1 {
                dir_down = !dir_down;
            }

            if dir_down {
                row += 1;
            } else {
                row -= 1;
            }

            col += 1;
        }

        rail
    }

    fn encrypt(&self, text: &str) -> String {
        let rail = self.create_rail_matrix(text);
        let mut result = String::new();

        for i in 0..self.key {
            for j in 0..text.len() {
                if let Some(c) = rail[i][j] {
                    result.push(c);
                }
            }
        }

        result
    }

    fn decrypt(&self, cipher: &str) -> String {
        let mut rail = self.create_rail_matrix(cipher);
        let mut index = 0;

        for i in 0..self.key {
            for j in 0..cipher.len() {
                if rail[i][j].is_some() {
                    rail[i][j] = cipher.chars().nth(index);
                    index += 1;
                }
            }
        }

        let mut result = String::new();
        let mut dir_down = false;
        let mut row = 0;
        let mut col = 0;

        for _ in 0..cipher.len() {
            if let Some(c) = rail[row][col] {
                result.push(c);
            }

            if row == 0 || row == self.key - 1 {
                dir_down = !dir_down;
            }

            if dir_down {
                row += 1;
            } else {
                row -= 1;
            }

            col += 1;
        }

        result
    }
}

fn test_rail_fence() {
    let test_cases = vec![
        ("Hello World!", 2),
        ("Programming is fun!", 3),
        ("Rust is amazing!", 4),
        ("Rail Fence Cipher", 5),
    ];

    for (i, (text, key)) in test_cases.iter().enumerate() {
        let cipher = RailFenceCipher::new(*key);

        let encrypted_text = cipher.encrypt(text);
        let decrypted_text = cipher.decrypt(&encrypted_text);

        println!("Test Case {}:", i + 1);
        println!("  Original Text: {}", text);
        println!("  Key: {}", key);
        println!("  Encrypted Text: {}", encrypted_text);
        println!("  Decrypted Text: {}", decrypted_text);

        let success = text == &decrypted_text;

        if success {
            println!("  Result: PASSED\n");
        } else {
            println!("  Result: FAILED\n");
        }
    }
}

fn main() {
    test_rail_fence();
}
