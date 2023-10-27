struct AnagramChecker;

impl AnagramChecker {
    fn new() -> AnagramChecker {
        AnagramChecker
    }

    fn calculate_frequency(&self, input: &str) -> [i32; 26] {
        let mut freq = [0; 26];

        for c in input.chars().filter(|c| c.is_ascii_alphabetic()) {
            let index = (c as u8 - b'a') as usize;
            freq[index] += 1;
        }

        freq
    }

    fn check(&self, a: &str, b: &str) -> bool {
        let freq_a = self.calculate_frequency(a);
        let freq_b = self.calculate_frequency(b);

        freq_a == freq_b
    }
}

fn test_check_anagram() {
    let anagram_checker = AnagramChecker::new();
    let test_cases = vec![
        ("listen", "silent", true),
        ("hello", "world", false),
        ("cinema", "iceman", true),
        ("rat", "car", false),
        ("abc", "def", false),
        ("aab", "bba", false),
        ("aabbcc", "abcabc", true),
        ("", "", true), // Empty strings are anagrams
    ];

    for (i, (a, b, expected)) in test_cases.iter().enumerate() {
        let result = anagram_checker.check(a, b);
        let status = if result == *expected { "PASSED" } else { "FAILED" };
        println!("Test Case {}: {}", i + 1, status);
        println!("  Input: '{}' and '{}'", a, b);
        println!("  Expected: {}", expected);
        println!("  Result: {}\n", result);
    }
}

fn main() {
    test_check_anagram();
}
