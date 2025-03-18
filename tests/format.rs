#![feature(allocator_api)]

use std::alloc::Global;
use std::fmt::Write;
use string_alloc::{format_in, String};

#[test]
fn test_format_macro() {
    let mut s = String::new_in(Global);
    write!(s, "Hello, {}!", "World").unwrap();
    assert_eq!(&*s, "Hello, World!");

    let mut s2 = String::new_in(Global);
    let name = "Alice";
    let age = 25;
    write!(s2, "{} is {} years old", name, age).unwrap();
    assert_eq!(&*s2, "Alice is 25 years old");

    // Test with UTF-8 characters
    let mut s3 = String::new_in(Global);
    write!(s3, "你好，{}！", "世界").unwrap();
    assert_eq!(&*s3, "你好，世界！");
}

#[test]
fn test_format_macro_direct() {
    let name = String::from_str_in("World", Global);
    let s = format_in!(Global, "Hello, {}!", name);
    assert_eq!(&*s, "Hello, World!");

    let name = String::from_str_in("Alice", Global);
    let age = 25;
    let s2 = format_in!(Global, "{} is {} years old", name, age);
    assert_eq!(&*s2, "Alice is 25 years old");

    // Test with UTF-8 characters
    let name = String::from_str_in("世界", Global);
    let s3 = format_in!(Global, "你好，{}！", name);
    assert_eq!(&*s3, "你好，世界！");
}

