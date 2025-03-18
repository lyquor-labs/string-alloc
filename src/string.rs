use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::ops::Deref;
use core::str;

use ::alloc::alloc::{Allocator, Global};

#[derive(Debug, Clone)]
pub struct String<A: Allocator + Clone + Default = Global> {
    vec: Vec<u8, A>,
}

impl<A: Allocator + Clone + Default> String<A> {
    /// Creates a new empty `String` with the specified allocator.
    ///
    /// See [`std::string::String::new`] for more details.
    pub fn new_in(alloc: A) -> Self {
        Self {
            vec: Vec::new_in(alloc),
        }
    }

    /// Creates a new empty `String` with at least the specified capacity with the specified allocator.
    ///
    /// See [`std::string::String::with_capacity`] for more details.
    pub fn with_capacity_in(cap: usize, alloc: A) -> Self {
        Self {
            vec: Vec::with_capacity_in(cap, alloc),
        }
    }

    /// Creates a new `String` from a string slice with the specified allocator.
    ///
    /// See [`std::string::String::from_str`] for more details.
    pub fn from_str_in(s: &str, alloc: A) -> Self {
        let mut vec = Vec::with_capacity_in(s.len(), alloc);
        vec.extend_from_slice(s.as_bytes());
        Self { vec }
    }

    /// Converts a vector of bytes to a `String` with the specified allocator.
    ///
    /// See [`std::string::String::from_utf8`] for more details.
    pub fn from_utf8_in(vec: Vec<u8, A>) -> Result<Self, core::str::Utf8Error> {
        match str::from_utf8(&vec) {
            Ok(_) => Ok(Self { vec }),
            Err(e) => Err(e),
        }
    }

    /// Converts a vector of bytes to a `String` with the specified allocator without checking that the string contains valid UTF-8.
    ///
    /// See [`std::string::String::from_utf8_unchecked`] for more details.
    pub unsafe fn from_utf8_unchecked_in(vec: Vec<u8, A>) -> Self {
        Self { vec }
    }

    /// Appends a given string slice onto the end of this `String`.
    ///
    /// See [`std::string::String::push_str`] for more details.
    pub fn push_str(&mut self, s: &str) {
        self.vec.extend_from_slice(s.as_bytes());
    }

    /// Appends the given char to the end of this `String`.
    ///
    /// See [`std::string::String::push`] for more details.
    pub fn push(&mut self, ch: char) {
        let mut buf = [0; 4];
        self.push_str(ch.encode_utf8(&mut buf));
    }

    /// Removes the last character from the string buffer and returns it.
    ///
    /// See [`std::string::String::pop`] for more details.
    pub fn pop(&mut self) -> Option<char> {
        let s = self.deref();
        let ch = s.chars().rev().next()?;
        let new_len = s.len() - ch.len_utf8();
        self.vec.truncate(new_len);
        Some(ch)
    }

    /// Inserts a character into this `String` at a byte position.
    ///
    /// See [`std::string::String::insert`] for more details.
    pub fn insert(&mut self, idx: usize, ch: char) {
        let mut buf = [0; 4];
        let bytes = ch.encode_utf8(&mut buf);
        let byte_idx = {
            let s = self.deref();
            s.char_indices()
                .nth(idx)
                .map(|(i, _)| i)
                .unwrap_or_else(|| panic!("insertion index (is {}) should be <= len (is {})", idx, s.len()))
        };
        self.vec.splice(byte_idx..byte_idx, bytes.as_bytes().iter().cloned());
    }

    /// Removes a char from this `String` at a byte position and returns it.
    ///
    /// See [`std::string::String::remove`] for more details.
    pub fn remove(&mut self, idx: usize) -> char {
        let (start, ch) = {
            let s = self.deref();
            s.char_indices()
                .nth(idx)
                .unwrap_or_else(|| panic!("removal index (is {}) should be < len (is {})", idx, s.chars().count()))
        };
        let end = start + ch.len_utf8();
        self.vec.drain(start..end);
        ch
    }

    /// Splits the string into two at the given byte index.
    ///
    /// See [`std::string::String::split_off`] for more details.
    pub fn split_off(&mut self, at: usize) -> Self
    where
        A: Clone,
    {
        let byte_idx = {
            let s = self.deref();
            s.char_indices().nth(at).map(|(i, _)| i).unwrap_or_else(|| {
                panic!(
                    "split_off index (is {}) should be <= len (is {})",
                    at,
                    s.chars().count()
                )
            })
        };
        let vec = self.vec.split_off(byte_idx);
        Self { vec }
    }

    /// Retains only the characters specified by the predicate.
    ///
    /// See [`std::string::String::retain`] for more details.
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(char) -> bool,
    {
        let mut i = 0;
        let mut len = self.len();
        while i < len {
            let ch = {
                let s = self.deref();
                match s[i..].chars().next() {
                    Some(c) => c,
                    None => break,
                }
            };
            let ch_len = ch.len_utf8();
            if !f(ch) {
                self.vec.drain(i..i + ch_len);
                len -= ch_len;
            } else {
                i += ch_len;
            }
        }
    }

    /// Ensures that this `String`'s capacity is at least `additional` bytes larger than its length.
    ///
    /// See [`std::string::String::reserve`] for more details.
    pub fn reserve(&mut self, additional: usize) {
        self.vec.reserve(additional);
    }

    /// Ensures that this `String`'s capacity is exactly `additional` bytes larger than its length.
    ///
    /// See [`std::string::String::reserve_exact`] for more details.
    pub fn reserve_exact(&mut self, additional: usize) {
        self.vec.reserve_exact(additional);
    }

    /// Shrinks the capacity of this `String` to match its length.
    ///
    /// See [`std::string::String::shrink_to_fit`] for more details.
    pub fn shrink_to_fit(&mut self) {
        self.vec.shrink_to_fit();
    }

    /// Truncates this `String`, removing all contents.
    ///
    /// See [`std::string::String::clear`] for more details.
    pub fn clear(&mut self) {
        self.vec.clear();
    }

    /// Shortens this `String` to the specified length.
    ///
    /// See [`std::string::String::truncate`] for more details.
    pub fn truncate(&mut self, new_len: usize) {
        let current_len = self.chars().count();
        if new_len > current_len {
            panic!("truncate index (is {}) should be <= len (is {})", new_len, current_len);
        }
        let byte_idx = self.char_indices().nth(new_len).map(|(i, _)| i).unwrap_or(self.len());
        self.vec.truncate(byte_idx);
    }

    /// Returns the length of this `String`, in bytes.
    ///
    /// See [`std::string::String::len`] for more details.
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    /// Returns the capacity of this `String`, in bytes.
    ///
    /// See [`std::string::String::capacity`] for more details.
    pub fn capacity(&self) -> usize {
        self.vec.capacity()
    }

    /// Converts the string into a new string with the specified allocator type.
    ///
    /// This method allows converting between different allocator types while preserving the string's contents.
    pub fn to_string_in<B: Allocator + Clone + Default>(&self) -> String<B> {
        String::from_str_in(self, B::default())
    }
}

impl<A: Allocator + Clone + Default> Deref for String<A> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        unsafe { str::from_utf8_unchecked(&self.vec) }
    }
}

impl<A: Allocator + Clone + Default> fmt::Display for String<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

impl<A: Allocator + Clone + Default> PartialEq<str> for String<A> {
    fn eq(&self, other: &str) -> bool {
        self.deref() == other
    }
}

impl<A: Allocator + Clone + Default> PartialEq for String<A> {
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}

impl<A: Allocator + Clone + Default> Eq for String<A> {}

impl<A: Allocator + Clone + Default> PartialOrd for String<A> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.deref().partial_cmp(other.deref())
    }
}

impl<A: Allocator + Clone + Default> Ord for String<A> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.deref().cmp(other.deref())
    }
}

impl<A: Allocator + Clone + Default> Hash for String<A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.deref().hash(state);
    }
}

impl<A: Allocator + Clone + Default> AsRef<str> for String<A> {
    fn as_ref(&self) -> &str {
        self.deref()
    }
}

impl<A: Allocator + Clone + Default> AsRef<[u8]> for String<A> {
    fn as_ref(&self) -> &[u8] {
        self.vec.as_ref()
    }
}

impl<A: Allocator + Clone + Default> Borrow<str> for String<A> {
    fn borrow(&self) -> &str {
        self.deref()
    }
}

impl<A: Allocator + Clone + Default> From<&str> for String<A> {
    fn from(s: &str) -> Self {
        Self::from_str_in(s, A::default())
    }
}

impl<A: Allocator + Clone + Default> From<Vec<u8, A>> for String<A> {
    fn from(vec: Vec<u8, A>) -> Self {
        Self { vec }
    }
}

impl<A: Allocator + Clone + Default> Into<Vec<u8, A>> for String<A> {
    fn into(self) -> Vec<u8, A> {
        self.vec
    }
}

// Add format! macro support
impl<A: Allocator + Clone + Default> fmt::Write for String<A> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.push_str(s);
        Ok(())
    }

    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
        // Pre-allocate capacity based on the format string length
        let capacity = args.as_str().map_or(0, |s| s.len());
        self.reserve(capacity);

        // Use the standard library's write_fmt implementation
        fmt::write(self, args)
    }
}

/// Creates a new `String` with the specified allocator and formats the arguments into it.
///
/// This macro is similar to the standard library's `format!` macro but returns our allocator-aware `String`.
///
/// # Examples
///
/// ```
/// #![feature(allocator_api)]
/// use string_alloc::{String, format_in};
/// use std::alloc::Global;
///
/// let name = "World";
/// let s = format_in!(Global, "Hello, {}!", name);
/// assert_eq!(&*s, "Hello, World!");
/// ```
#[macro_export]
macro_rules! format_in {
    ($alloc:expr, $($arg:tt)*) => {{
        use std::fmt::Write;
        let mut s = $crate::String::new_in($alloc);
        write!(s, $($arg)*).unwrap();
        s
    }};
}

// Add conversions to/from std::string::String
#[cfg(feature = "std")]
impl<A: Allocator + Clone + Default> From<std::string::String> for String<A> {
    fn from(s: std::string::String) -> Self {
        Self::from_str_in(&s, A::default())
    }
}

#[cfg(feature = "std")]
impl<A: Allocator + Clone + Default> From<String<A>> for std::string::String {
    fn from(s: String<A>) -> Self {
        Self::from(&*s)
    }
}

// Add serde support
#[cfg(feature = "serde")]
impl<A: Allocator + Clone + Default> serde::Serialize for String<A> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self)
    }
}

#[cfg(feature = "serde")]
impl<'de, A: Allocator + Clone + Default> serde::Deserialize<'de> for String<A> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;
        Ok(Self::from_str_in(s, A::default()))
    }
}

impl<A: Allocator + Clone + Default> core::ops::Add<&str> for String<A> {
    type Output = Self;

    fn add(mut self, other: &str) -> Self {
        self.push_str(other);
        self
    }
}
