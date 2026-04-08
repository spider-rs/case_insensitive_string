use case_insensitive_string::CaseInsensitiveString;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

fn hash_of(s: &CaseInsensitiveString) -> u64 {
    let mut h = DefaultHasher::new();
    s.hash(&mut h);
    h.finish()
}

#[test]
fn hash_case_insensitive() {
    let a = CaseInsensitiveString::new("Hello World");
    let b = CaseInsensitiveString::new("hello world");
    let c = CaseInsensitiveString::new("HELLO WORLD");
    let d = CaseInsensitiveString::new("hElLo WoRlD");
    assert_eq!(hash_of(&a), hash_of(&b));
    assert_eq!(hash_of(&a), hash_of(&c));
    assert_eq!(hash_of(&a), hash_of(&d));
}

#[test]
fn hash_different_strings_differ() {
    let a = CaseInsensitiveString::new("hello");
    let b = CaseInsensitiveString::new("world");
    assert_ne!(hash_of(&a), hash_of(&b));
}

#[test]
fn hash_empty_string() {
    let a = CaseInsensitiveString::new("");
    let b = CaseInsensitiveString::new("");
    assert_eq!(hash_of(&a), hash_of(&b));
}

#[test]
fn hash_non_ascii_passthrough() {
    // Non-ASCII bytes should pass through unchanged (only ASCII A-Z are folded).
    let a = CaseInsensitiveString::new("cafe\u{0301}"); // café with combining accent
    let b = CaseInsensitiveString::new("cafe\u{0301}");
    assert_eq!(hash_of(&a), hash_of(&b));
}

#[test]
fn hash_url_realistic() {
    let a = CaseInsensitiveString::new("https://Example.COM/Blog/Post-Title?Ref=HOME");
    let b = CaseInsensitiveString::new("https://example.com/blog/post-title?ref=home");
    assert_eq!(hash_of(&a), hash_of(&b));
}

#[test]
fn hash_long_string_simd_path() {
    // 64 bytes — well into the SIMD path.
    let a = CaseInsensitiveString::new(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789abcdefghijklmnopqrstuvwxyz!!",
    );
    let b = CaseInsensitiveString::new(
        "abcdefghijklmnopqrstuvwxyz0123456789abcdefghijklmnopqrstuvwxyz!!",
    );
    assert_eq!(hash_of(&a), hash_of(&b));
}

#[test]
fn hash_exactly_16_bytes() {
    let a = CaseInsensitiveString::new("ABCDEFGHabcdefgh");
    let b = CaseInsensitiveString::new("abcdefghabcdefgh");
    assert_eq!(hash_of(&a), hash_of(&b));
}

#[test]
fn hash_17_bytes_simd_plus_tail() {
    let a = CaseInsensitiveString::new("ABCDEFGHIJKLMNOPQ");
    let b = CaseInsensitiveString::new("abcdefghijklmnopq");
    assert_eq!(hash_of(&a), hash_of(&b));
}

#[test]
fn hash_15_bytes_scalar_only() {
    let a = CaseInsensitiveString::new("ABCDEFGHIJKLMNO");
    let b = CaseInsensitiveString::new("abcdefghijklmno");
    assert_eq!(hash_of(&a), hash_of(&b));
}

#[test]
fn hashset_case_insensitive_dedup() {
    let mut set = HashSet::new();
    set.insert(CaseInsensitiveString::new("https://Example.COM"));
    set.insert(CaseInsensitiveString::new("https://example.com"));
    set.insert(CaseInsensitiveString::new("https://EXAMPLE.COM"));
    // All three should be treated as the same key.
    assert_eq!(set.len(), 1);
}

#[test]
fn hashset_contains_case_insensitive() {
    let mut set = HashSet::new();
    set.insert(CaseInsensitiveString::new("Hello"));
    assert!(set.contains(&CaseInsensitiveString::new("hello")));
    assert!(set.contains(&CaseInsensitiveString::new("HELLO")));
    assert!(set.contains(&CaseInsensitiveString::new("hElLo")));
    assert!(!set.contains(&CaseInsensitiveString::new("world")));
}

#[test]
fn ord_case_insensitive() {
    let a = CaseInsensitiveString::new("Alpha");
    let b = CaseInsensitiveString::new("beta");
    assert!(a < b);

    let c = CaseInsensitiveString::new("ALPHA");
    assert_eq!(a.cmp(&c), std::cmp::Ordering::Equal);
}

#[test]
fn ord_prefix() {
    let a = CaseInsensitiveString::new("abc");
    let b = CaseInsensitiveString::new("ABCD");
    assert!(a < b);
}

#[test]
fn ord_empty() {
    let a = CaseInsensitiveString::new("");
    let b = CaseInsensitiveString::new("");
    assert_eq!(a.cmp(&b), std::cmp::Ordering::Equal);
    assert!(a < CaseInsensitiveString::new("a"));
}

#[test]
fn hash_consistency_across_lengths() {
    // Ensure the hash contract holds: a == b implies hash(a) == hash(b)
    // for various lengths that exercise different code paths.
    for len in 0..=64 {
        let upper: String = (0..len).map(|i| (b'A' + (i % 26) as u8) as char).collect();
        let lower: String = upper.to_ascii_lowercase();
        let a = CaseInsensitiveString::new(&upper);
        let b = CaseInsensitiveString::new(&lower);
        assert_eq!(
            hash_of(&a),
            hash_of(&b),
            "Hash mismatch at len={}: {:?} vs {:?}",
            len,
            upper,
            lower
        );
    }
}
