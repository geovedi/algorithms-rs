/**
 * Adler Hash algorithms
 */

fn adler32(s: &str) -> u32 {
    let mut a: u32 = 1;
    let mut b: u32 = 0;
    const MODADLER: u32 = 65521;

    for byte in s.bytes() {
        a = (a + byte as u32) % MODADLER;
        b = (b + a) % MODADLER;
    }

    (b << 16) | a
}

fn test_adler32() {
    assert_eq!(adler32("Hello World"), 403375133);
    assert_eq!(adler32("Hello World!"), 474547262);
    assert_eq!(adler32("Hello world"), 413860925);
    assert_eq!(adler32("Hello world!"), 487130206);
    println!("Tests passed");
}

fn main() {
    test_adler32();
}
