struct Rot13Cipher;

impl Rot13Cipher {
    fn new() -> Rot13Cipher {
        Rot13Cipher
    }

    fn apply(&self, s: &str) -> String {
        let mut result = String::new();

        for char in s.chars() {
            let mut byte = char as u8;
            
            if byte >= b'A' && byte <= b'Z' {
                byte = b'A' + (byte - b'A' + 13) % 26;
            } else if byte >= b'a' && byte <= b'z' {
                byte = b'a' + (byte - b'a' + 13) % 26;
            }
            
            result.push(char::from(byte));
        }

        result
    }
}

fn test_rot13() {
    let cipher = Rot13Cipher::new();

    let test_cases = vec![
        ("The more I C, the less I see.", "Gur zber V P, gur yrff V frr."),
        ("Which witch switched the Swiss wristwatches?", "Juvpu jvgpu fjvgpurq gur Fjvff jevfgjngpurf?"),
        ("Juvpu jvgpu fjvgpurq gur Fjvff jevfgjngpurf?", "Which witch switched the Swiss wristwatches?"),
    ];

    for (input, expected) in test_cases {
        assert_eq!(cipher.apply(input), expected);
    }

    println!("All tests have successfully passed!");
}

fn main() {
    test_rot13();
}
