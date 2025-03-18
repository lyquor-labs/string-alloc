#![feature(allocator_api)]

use std::alloc::Global;
use std::hash::Hash;
use std::string::String as StdString;
use string_alloc::String;

#[test]
fn test_basic_construction() {
    let _s1 = String::from_str_in("hello", Global);
    let s2 = String::with_capacity_in(128, Global);
    assert!(s2.capacity() >= 128);
}

#[test]
fn test_from_conversions() {
    let s3: String = From::from("xyz");
    let v: Vec<u8, _> = s3.into();
    assert_eq!(v, b"xyz");
}

#[test]
fn test_utf8_conversions() {
    let raw = Vec::from("valid utf8".as_bytes());
    let s4 = String::from_utf8_in(raw.clone()).unwrap();
    assert_eq!(&*s4, "valid utf8");

    let mut invalid = raw.clone();
    invalid.push(0xFF);
    let result = String::from_utf8_in(invalid);
    assert!(result.is_err());

    let unchecked = unsafe { String::from_utf8_unchecked_in(raw.clone()) };
    assert_eq!(&*unchecked, "valid utf8");
}

#[test]
fn test_string_manipulation() {
    let mut s = String::from_str_in("hello", Global);

    // Basic string operations
    s.push_str(", world");
    s.push('!');
    assert_eq!(&*s, "hello, world!");

    // Character operations
    let ch = s.pop();
    assert_eq!(ch, Some('!'));
    s.insert(5, '-');
    assert_eq!(&*s, "hello-, world");
    let r = s.remove(5);
    assert_eq!(r, '-');
    assert_eq!(&*s, "hello, world");

    // Splitting
    let right = s.split_off(5);
    assert_eq!(&*s, "hello");
    assert_eq!(&*right, ", world");

    // Retaining
    let mut s4 = String::from_str_in("a1b2c3", Global);
    s4.retain(|c| c.is_alphabetic());
    assert_eq!(&*s4, "abc");
}

#[test]
fn test_edge_cases() {
    let mut s = String::from_str_in("", Global);

    // Test empty string operations
    assert_eq!(s.pop(), None);
    assert_eq!(s.len(), 0);

    // Test single character operations
    s.push('a');
    assert_eq!(s.pop(), Some('a'));

    // Test UTF-8 boundary operations
    s.push_str("你好");
    assert_eq!(s.len(), 6); // 2 UTF-8 characters
    assert_eq!(s.pop(), Some('好'));
    assert_eq!(s.pop(), Some('你'));

    // Test capacity edge cases
    s.reserve(0);
    s.reserve_exact(0);
    s.shrink_to_fit();
}

#[test]
fn test_capacity_edge_cases() {
    let mut s = String::from_str_in("hello", Global);

    // Test zero capacity
    s.reserve(0);
    s.reserve_exact(0);

    // Test large capacity
    s.reserve(1000);
    assert!(s.capacity() >= 1005); // 5 + 1000

    // Test exact capacity
    s.reserve_exact(100);
    assert!(s.capacity() >= 105); // 5 + 100

    // Test truncation edge cases
    s.truncate(0);
    assert_eq!(s.len(), 0);
}

#[test]
fn test_traits_and_deref() {
    // Equality and ordering
    let s1 = String::from_str_in("abc", Global);
    let s2 = String::from_str_in("abc", Global);
    assert_eq!(format!("{}", s1), "abc");
    assert_eq!(s1, s2);
    assert!(s1 <= s2);

    // Hashing
    use std::collections::hash_map::DefaultHasher;
    let mut hasher = DefaultHasher::new();
    s1.hash(&mut hasher);

    // Borrowing and references
    let asref: &str = s1.as_ref();
    assert_eq!(asref, "abc");

    // String slice operations (through Deref)
    let s3 = String::from_str_in("hello world", Global);

    // Character iteration
    let chars: Vec<char> = s3.chars().collect();
    assert_eq!(chars, vec!['h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd']);

    // Character indices
    let indices: Vec<(usize, char)> = s3.char_indices().collect();
    assert_eq!(indices[0], (0, 'h'));
    assert_eq!(indices[5], (5, ' '));

    // Byte iteration
    let bytes: Vec<u8> = s3.bytes().collect();
    assert_eq!(bytes, b"hello world");

    // String operations through Deref
    assert!(s3.contains("hello"));
    assert!(!s3.contains("xyz"));
    assert!(s3.starts_with("hello"));
    assert!(s3.ends_with("world"));

    // Slicing through Deref
    let (first, second) = s3.split_at(6);
    assert_eq!(first, "hello ");
    assert_eq!(second, "world");
}

#[test]
fn test_utf8_edge_cases() {
    // Test various UTF-8 character lengths
    let mut s = String::from_str_in("", Global);

    // 1-byte UTF-8
    s.push('a');
    assert_eq!(s.len(), 1);

    // 2-byte UTF-8
    s.push('é');
    assert_eq!(s.len(), 3);

    // 3-byte UTF-8
    s.push('中');
    assert_eq!(s.len(), 6);

    // 4-byte UTF-8
    s.push('🦀');
    assert_eq!(s.len(), 10);

    // Test UTF-8 boundaries
    let mut s2 = String::from_str_in("", Global);
    s2.push_str("aé中🦀");
    assert_eq!(s2.len(), 10);

    // Test character boundaries
    assert_eq!(s2.chars().count(), 4);
    assert_eq!(s2.bytes().count(), 10);

    // Test character indices
    let indices: Vec<(usize, char)> = s2.char_indices().collect();
    assert_eq!(indices[0], (0, 'a'));
    assert_eq!(indices[1], (1, 'é'));
    assert_eq!(indices[2], (3, '中'));
    assert_eq!(indices[3], (6, '🦀'));
}

#[test]
fn test_allocator_edge_cases() {
    // Test with zero capacity
    let s1 = String::with_capacity_in(0, Global);
    assert_eq!(s1.capacity(), 0);

    // Test with large capacity
    let s2 = String::with_capacity_in(1024 * 1024, Global);
    assert!(s2.capacity() >= 1024 * 1024);

    // Test capacity growth
    let mut s3 = String::from_str_in("", Global);
    let initial_cap = s3.capacity();
    s3.push_str("x".repeat(initial_cap + 1).as_str());
    assert!(s3.capacity() > initial_cap);

    // Test capacity shrinking
    let mut s4 = String::from_str_in("hello", Global);
    s4.reserve(1000);
    let large_cap = s4.capacity();
    s4.shrink_to_fit();
    assert!(s4.capacity() < large_cap);
}

#[test]
fn test_complex_operations() {
    let mut s = String::from_str_in("", Global);

    // Test multiple operations in sequence
    s.push_str("Hello");
    s.push(' ');
    s.push_str("World");
    assert_eq!(&*s, "Hello World");

    // Test string manipulation with UTF-8
    s.insert(6, '🦀');
    assert_eq!(&*s, "Hello 🦀World");

    // Test complex retain
    s.retain(|c| c.is_alphabetic() || c == '🦀');
    assert_eq!(&*s, "Hello🦀World");

    // Test multiple splits
    let mut s2 = s.clone();
    let mut right = s2.split_off(5);
    let right2 = right.split_off(1);
    assert_eq!(&*s2, "Hello");
    assert_eq!(&*right2, "World");

    // Test capacity management during operations
    let mut s3 = String::from_str_in("", Global);
    s3.reserve(100);
    let cap1 = s3.capacity();
    s3.push_str("x".repeat(50).as_str());
    s3.shrink_to_fit();
    let cap2 = s3.capacity();
    assert!(cap2 < cap1);
}

#[test]
fn test_workarounds() {
    let s = String::from_str_in("Hello World", Global);

    // Workaround for drain-like operations
    // 1. Using split_off for ranges
    let mut s2 = s.clone();
    let drained = s2.split_off(6); // "World"
    assert_eq!(&*s2, "Hello ");
    assert_eq!(&*drained, "World");

    // 2. Using retain for filtering
    let mut s3 = s.clone();
    s3.retain(|c| c != 'l');
    assert_eq!(&*s3, "Heo Word");

    // Workaround for get-like operations
    // 1. Using string slicing through &str
    let world = &s[6..]; // "World"
    assert_eq!(world, "World");

    // 2. Using split_at
    let (hello, world) = s.split_at(6);
    assert_eq!(hello, "Hello ");
    assert_eq!(world, "World");

    // Workaround for get_mut-like operations
    // 1. Using remove and insert
    let mut s4 = s.clone();
    let _c = s4.remove(1); // 'e'
    s4.insert(1, 'E');
    assert_eq!(&*s4, "HEllo World");

    // 2. Using split_off and push_str
    let mut s5 = s.clone();
    let _right = s5.split_off(6);
    s5.push_str("Rust");
    assert_eq!(&*s5, "Hello Rust");

    // Complex example combining multiple operations
    let s6 = String::from_str_in("Hello World", Global);

    // Replace a substring (like get_mut for a range)
    let (left, _right) = s6.split_at(6);
    let mut new = String::from_str_in(left, Global);
    new.push_str("Rust");
    assert_eq!(&*new, "Hello Rust");

    // Filter a range (like drain with a predicate)
    let s7 = String::from_str_in("Hello World", Global);
    let (left, right) = s7.split_at(6);
    let mut filtered = String::from_str_in(left, Global);
    let filtered_chars: Vec<char> = right.chars().filter(|&c| c != 'l').collect();
    let temp_string: StdString = filtered_chars.into_iter().collect();
    let filtered_right = String::from_str_in(&temp_string, Global);
    filtered.push_str(&filtered_right);
    assert_eq!(&*filtered, "Hello Word");
}
