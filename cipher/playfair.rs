struct PlayfairCipher {
    key: String,
    matrix: Vec<Vec<char>>,
}

impl PlayfairCipher {
    fn new(key: &str) -> PlayfairCipher {
        let key = key.to_uppercase().replace("J", "I");
        let matrix = PlayfairCipher::generate_matrix(&key);
        PlayfairCipher { key, matrix }
    }

    fn generate_matrix(key: &str) -> Vec<Vec<char>> {
        let alphabet = "ABCDEFGHIKLMNOPQRSTUVWXYZ";
        let key = key.to_uppercase().replace("J", "I");
        let mut seen = [false; 26];
        let mut matrix = Vec::with_capacity(5);
        let mut row = Vec::with_capacity(5);

        fn add_char(c: char, seen: &mut [bool; 26], row: &mut Vec<char>, matrix: &mut Vec<Vec<char>>) {
            let index = c as usize - 'A' as usize;
            if !seen[index] {
                row.push(c);
                seen[index] = true;
                if row.len() == 5 {
                    matrix.push(row.clone());
                    row.clear();
                }
            }
        }

        for c in key.chars() {
            add_char(c, &mut seen, &mut row, &mut matrix);
        }

        for c in alphabet.chars() {
            add_char(c, &mut seen, &mut row, &mut matrix);
        }

        matrix
    }

    fn encrypt(&self, plaintext: &str) -> String {
        let plaintext = plaintext
            .to_uppercase()
            .replace("J", "I")
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .collect::<String>();
        let mut ciphertext = String::new();

        fn find_position(matrix: &Vec<Vec<char>>, c: char) -> (usize, usize) {
            for (row, row_vec) in matrix.iter().enumerate() {
                for (col, &ch) in row_vec.iter().enumerate() {
                    if ch == c {
                        return (row, col);
                    }
                }
            }
            (0, 0) // Fallback, should not happen with valid input
        }

        fn encrypt_pair(matrix: &Vec<Vec<char>>, a: char, b: char) -> String {
            let (mut row1, mut col1) = find_position(matrix, a);
            let (mut row2, mut col2) = find_position(matrix, b);

            if row1 == row2 {
                col1 = (col1 + 1) % 5;
                col2 = (col2 + 1) % 5;
            } else if col1 == col2 {
                row1 = (row1 + 1) % 5;
                row2 = (row2 + 1) % 5;
            } else {
                let temp = col1;
                col1 = col2;
                col2 = temp;
            }

            format!("{}{}", matrix[row1][col1], matrix[row2][col2])
        }

        let mut i = 0;
        while i < plaintext.len() {
            let a = plaintext.chars().nth(i).unwrap();
            let b = if i + 1 < plaintext.len() {
                plaintext.chars().nth(i + 1).unwrap()
            } else {
                'X'
            };

            let b = if a == b { 'X' } else { b };

            ciphertext.push_str(&encrypt_pair(&self.matrix, a, b));
            i += 2;
        }

        ciphertext
    }
}

fn main() {
    let cipher = PlayfairCipher::new("KEY");
    assert_eq!(cipher.encrypt("HELLO WORLD"), "DBNVMZMTQL");
    assert_eq!(cipher.encrypt("PLAYFAIR CIPHER"), "QIBAGYMPIPTCYQ");
    assert_eq!(cipher.encrypt("ABCD"), "BKDF");
    println!("All tests PASSED");
}
