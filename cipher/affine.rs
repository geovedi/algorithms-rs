const ALPHABET_SIZE: u8 = 95;
const Z95_CONVERSION_CONSTANT: u8 = 32;

fn modular_inverse(a: i32, m: i32) -> Result<i32, &'static str> {
    let mut x0 = 1;
    let mut x1 = 0;
    let mut original_m = m;

    let mut a = a;
    let mut m = m;

    while m != 0 {
        let q = a / m;
        let r = a % m;
        let next_x = x0 - q * x1;

        a = m;
        m = r;
        x0 = x1;
        x1 = next_x;
    }

    if a > 1 {
        return Err("Inverse does not exist");
    }

    if x0 < 0 {
        x0 += original_m;
    }

    Ok(x0)
}

struct AffineCipher {
    a: i32,
    b: i32,
}

impl AffineCipher {
    fn new(a: i32, b: i32) -> AffineCipher {
        AffineCipher { a, b }
    }

    fn encrypt(&self, plaintext: &str) -> String {
        let mut ciphertext = String::new();
        for c in plaintext.chars() {
            let char_code = c as i32 - Z95_CONVERSION_CONSTANT as i32;
            let char_code = ((char_code * self.a) + self.b) % ALPHABET_SIZE as i32;
            ciphertext.push((char_code + Z95_CONVERSION_CONSTANT as i32) as u8 as char);
        }
        ciphertext
    }

    fn decrypt(&self, ciphertext: &str) -> Result<String, &'static str> {
        let a_inverse = modular_inverse(self.a, ALPHABET_SIZE as i32)?;
        let b_inverse = -(self.b % ALPHABET_SIZE as i32) + ALPHABET_SIZE as i32;
        let mut plaintext = String::new();

        for c in ciphertext.chars() {
            let char_code = c as i32 - Z95_CONVERSION_CONSTANT as i32;
            let char_code = (a_inverse * (char_code + b_inverse)) % ALPHABET_SIZE as i32;
            plaintext.push((char_code + Z95_CONVERSION_CONSTANT as i32) as u8 as char);
        }

        Ok(plaintext)
    }
}

fn test_affine() {
    let test_cases = vec![
        ("Hello!", "&3ddy2", 7, 11),
        ("TheAlgorithms/C", "DNC}=jHS2zN!7;E", 67, 67),
        ("0123456789", "840,($ {ws", 91, 88),
        ("7W@;cdeRT9uL", "JDfa*we?z&bL", 77, 76),
        ("~Qr%^-+++$leM", "r'qC0$sss;Ahf", 8, 90),
        (
            "The quick brown fox jumps over the lazy dog",
            "K7: .*6<4 =-0(1 90' 5*2/, 0):- +7: 3>%& ;08",
            94,
            0,
        ),
        (
            "One-1, Two-2, Three-3, Four-4, Five-5, Six-6, Seven-7, Eight-8, Nine-9, Ten-10",
            "H&60>\\2*uY0q\\2*p4660E\\2XYn40x\\2XDB60L\\2VDI0 \\2V6B6&0S\\2%D=p;0'\\2tD&60Z\\2*6&0>j",
            51,
            18,
        ),
    ];

    for (plaintext, expected_ciphertext, a, b) in test_cases {
        let cipher = AffineCipher::new(a, b);
        let ciphertext = cipher.encrypt(plaintext);
        assert_eq!(ciphertext, expected_ciphertext);

        let decrypted_plaintext = cipher.decrypt(&ciphertext);
        assert_eq!(decrypted_plaintext, Ok(plaintext.to_string()));
    }

    println!("All tests have successfully passed!");
}

fn main() {
    test_affine();
}
