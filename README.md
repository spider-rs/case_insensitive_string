# case_insensitive_string

A rust case_insensitive_string struct crate.

Install the crate with `cargo add case_insensitive_string`.

```rust
use case_insensitive_string::CaseInsensitiveString;

fn main() {
    let case_insensitive = CaseInsensitiveString::from("iDk");

    // both of the strings are a match!
    assert_eq!(case_insensitive, CaseInsensitiveString::from("IDK"))
}
```

## Features

1. `compact` feature flag to enable [compact_str](https://github.com/ParkMyCar/compact_str) usage.
1. `serde` feature flag to enable [serde](https://github.com/serde-rs/serde) usage.