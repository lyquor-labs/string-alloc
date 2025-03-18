# string-alloc

An allocator-aware, `no_std`-compatible implementation of `String<A>` that mirrors `std::string::String`.

## Features

- UTF-8 correctness
- Full `no_std` support via `extern crate alloc`
- Custom allocator compatibility

## Usage

```rust
use string_alloc::String;
use std::alloc::Global;

let mut s = String::from_str_in("hello", Global);
s.push_str(" world");
```

## License

Apache-2.0
