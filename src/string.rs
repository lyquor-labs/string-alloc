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
    pub fn new_in(alloc: A) -> Self {
        Self {
            vec: Vec::new_in(alloc),
        }
    }

    pub fn with_capacity_in(cap: usize, alloc: A) -> Self {
        Self {
            vec: Vec::with_capacity_in(cap, alloc),
        }
    }

    pub fn from_str_in(s: &str, alloc: A) -> Self {
        let mut vec = Vec::with_capacity_in(s.len(), alloc);
        vec.extend_from_slice(s.as_bytes());
        Self { vec }
    }

    pub fn from_utf8_in(vec: Vec<u8, A>) -> Result<Self, core::str::Utf8Error> {
        match str::from_utf8(&vec) {
            Ok(_) => Ok(Self { vec }),
            Err(e) => Err(e),
        }
    }

    pub unsafe fn from_utf8_unchecked_in(vec: Vec<u8, A>) -> Self {
        Self { vec }
    }

    pub fn push_str(&mut self, s: &str) {
        self.vec.extend_from_slice(s.as_bytes());
    }

    pub fn push(&mut self, ch: char) {
        let mut buf = [0; 4];
        self.push_str(ch.encode_utf8(&mut buf));
    }

    pub fn pop(&mut self) -> Option<char> {
        let s = self.deref();
        let ch = s.chars().rev().next()?;
        let new_len = s.len() - ch.len_utf8();
        self.vec.truncate(new_len);
        Some(ch)
    }

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

    pub fn reserve(&mut self, additional: usize) {
        self.vec.reserve(additional);
    }

    pub fn reserve_exact(&mut self, additional: usize) {
        self.vec.reserve_exact(additional);
    }

    pub fn shrink_to_fit(&mut self) {
        self.vec.shrink_to_fit();
    }

    pub fn clear(&mut self) {
        self.vec.clear();
    }

    pub fn truncate(&mut self, new_len: usize) {
        let current_len = self.chars().count();
        if new_len > current_len {
            panic!("truncate index (is {}) should be <= len (is {})", new_len, current_len);
        }
        let byte_idx = self.char_indices().nth(new_len).map(|(i, _)| i).unwrap_or(self.len());
        self.vec.truncate(byte_idx);
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn capacity(&self) -> usize {
        self.vec.capacity()
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
