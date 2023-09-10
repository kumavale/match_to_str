# match_to_str

[![Actions Status](https://github.com/kumavale/match_to_str/workflows/Rust/badge.svg)](https://github.com/kumavale/match_to_str/actions)
[![Crate](https://img.shields.io/crates/v/match_to_str.svg)](https://crates.io/crates/match_to_str)
[![license](https://img.shields.io/badge/License-MIT-blue.svg?style=flat)](LICENSE-MIT)
[![license](https://img.shields.io/badge/License-Apache%202.0-blue.svg?style=flat)](LICENSE-APACHE)

Converts the pattern part of the `match` expression into a `&'static str`.  
It is mainly intended to be used when you want to convert a variable name into a string as is.

## Usage

```
$ cargo add match_to_str
```

```rust
use match_to_str::match_to_str;

const CAT: u8 = 1;
const DOG: u8 = 2;

fn main() {
    let animal = match_to_str!(1 => {
        CAT,
        DOG,
        _,
    });
    assert_eq!(animal, "CAT");
}
```

expand to:

```rust
let animal = match 1 {
    CAT => "CAT",
    DOG => "DOG",
    _ => "_",
};
```

## Contributing

This project welcomes your PR and issues. For example, fixing bugs, adding features, refactoring, etc.
