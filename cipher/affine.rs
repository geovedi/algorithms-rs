/**
 * An affine cipher is a letter substitution cipher that uses a linear transformation to substitute
 * letters in a message.
 * Given an alphabet of length M with characters with numeric values 0-(M-1), an arbitrary character x can be transformed
 * with the expression (ax + b) % M into our ciphertext character. The only caveat is that a must be
 * relatively prime with M in order for this transformation to be invertible, i.e., gcd(a, M) = 1.
 */

/**
 * Number of characters in our alphabet (26 English alphabet letters)
 */
const ALPHABET_SIZE: i32 = 26;

/**
 * A structure representing an affine cipher key
 */
struct AffineKey {
    a: i32,  // what the character is being multiplied by
    b: i32,  // what is being added after the multiplication with `a`
}

/**
 * Encrypts character string `s` with key
 */
fn affine_encrypt(s: &mut String, key: &AffineKey) {
    let m = ALPHABET_SIZE as i32;
    let encrypted = s.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let x = if c.is_ascii_lowercase() {
                c as i32 - 'a' as i32
            } else {
                c as i32 - 'A' as i32
            };
            let encrypted_char = (key.a * x + key.b) % m;
            let base = if c.is_ascii_lowercase() { 'a' } else { 'A' } as i32;
            (encrypted_char + base) as u8 as char
        } else {
            c
        }
    });
    *s = encrypted.collect();
}

/**
 * Decrypts an affine ciphertext
 */
fn affine_decrypt(s: &mut String, key: &AffineKey) {
    let m = ALPHABET_SIZE as i32;
    let a_inverse = mod_inverse(key.a, m);
    let decrypted = s.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let x = if c.is_ascii_lowercase() {
                c as i32 - 'a' as i32
            } else {
                c as i32 - 'A' as i32
            };
            let mut decrypted_char = a_inverse * (x - key.b) % m;
            if decrypted_char < 0 {
                decrypted_char += m; // Ensure the result is non-negative
            }
            let base = if c.is_ascii_lowercase() { 'a' } else { 'A' } as i32;
            (decrypted_char + base) as u8 as char
        } else {
            c
        }
    });
    *s = decrypted.collect();
}

/**
 * Calculate the modular inverse of a number `a` modulo `m`
 */
fn mod_inverse(a: i32, m: i32) -> i32 {
    for x in 1..m {
        if (a * x) % m == 1 {
            return x;
        }
    }
    1 // Default to 1 if no modular inverse exists
}

/**
 * Test multiple strings
 */
fn tests() {
    test_string("Hello!", "Inkkf!", 7, 11);
    test_string("TheAlgorithms/C", "OqxPybrkfoqnz/T", 67, 67);
    test_string("0123456789", "0123456789", 91, 88);
    test_string("7W@;cdeRT9uL", "7C@;wvuHF9eN", 77, 76);
    test_string(
        "One-1, Two-2, Three-3, Four-4, Five-5, Six-6, Seven-7, Eight-8, Nine-9, Ten-10",
        "One-1, Two-2, Three-3, Four-4, Five-5, Six-6, Seven-7, Eight-8, Nine-9, Ten-10",
        1, 0,
    );

    println!("All tests have successfully passed!");
}

fn test_string(s: &str, ciphertext: &str, a: i32, b: i32) {
    let mut copy = s.to_string();
    let key = AffineKey { a, b };

    affine_encrypt(&mut copy, &key);
    assert_eq!(copy, ciphertext);  // assert that the encryption worked

    affine_decrypt(&mut copy, &key);
    assert_eq!(copy, s);  // assert that we got the same string we started with
}

fn main() {
    tests();
}
