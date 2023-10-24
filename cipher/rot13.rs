/**
 * ROT13 is a simple letter substitution cipher that replaces a letter with 
 * the 13th letter after it in the alphabet.
 * ROT13 transforms a piece of text by examining its alphabetic characters and 
 * replacing each one with the letter 13 places further along in the alphabet, 
 * wrapping back to the beginning if necessary. A becomes N, B becomes O, 
 * and so on up to M, which becomes Z, then the sequence continues at
 * the beginning of the alphabet: N becomes A, O becomes B, and so on to Z,
 * which becomes M.
 */

fn rot13(s: &mut String) {
    s.chars_mut().for_each(|c| {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            c.replace_range(
                ..1,
                &(((c as u8 - base + 13) % 26 + base) as char).to_string(),
            );
        }
    });
}

fn test() {
    let mut test_01 = String::from("The more I C, the less I see.");
    rot13(&mut test_01);
    assert_eq!(test_01, "Gur zber V P, gur yrff V frr.");

    let mut test_02 = String::from("Which witch switched the Swiss wristwatches?");
    rot13(&mut test_02);
    assert_eq!(test_02, "Juvpu jvgpu fjvgpurq gur Fjvff jevfgjngpurf?");

    let mut test_03 = String::from("Juvpu jvgpu fjvgpurq gur Fjvff jevfgjngpurf?");
    rot13(&mut test_03);
    assert_eq!(test_03, "Which witch switched the Swiss wristwatches?");

    println!("All tests have successfully passed!");
}

fn main() {
    test(); // run self-test implementations
}
