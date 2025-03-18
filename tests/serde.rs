#![feature(allocator_api)]

#[cfg(feature = "serde")]
use string_alloc::String;
#[cfg(feature = "serde")]
use std::alloc::Global;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
#[test]
fn test_serde() {
    // Test basic serialization/deserialization
    let s = String::from_str_in("Hello, World!", Global);
    let serialized = serde_json::to_string(&s).unwrap();
    assert_eq!(serialized, "\"Hello, World!\"");

    let deserialized: String<Global> = serde_json::from_str(&serialized).unwrap();
    assert_eq!(&*deserialized, "Hello, World!");

    // Test with UTF-8 characters
    let s2 = String::from_str_in("你好，世界！", Global);
    let serialized2 = serde_json::to_string(&s2).unwrap();
    assert_eq!(serialized2, "\"你好，世界！\"");

    let deserialized2: String<Global> = serde_json::from_str(&serialized2).unwrap();
    assert_eq!(&*deserialized2, "你好，世界！");

    // Test in a struct
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Person {
        name: String<Global>,
        message: String<Global>,
    }

    let person = Person {
        name: String::from_str_in("Alice", Global),
        message: String::from_str_in("Hello, World!", Global),
    };

    let serialized = serde_json::to_string(&person).unwrap();
    let deserialized: Person = serde_json::from_str(&serialized).unwrap();
    assert_eq!(person, deserialized);
} 