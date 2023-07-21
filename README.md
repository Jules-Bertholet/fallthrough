# `fallthrough`

[![Build Status](https://github.com/Jules-Bertholet/fallthrough/actions/workflows/actions.yml/badge.svg)](https://github.com/Jules-Bertholet/fallthrough/actions) [![API reference](https://img.shields.io/docsrs/fallthrough)](https://docs.rs/fallthrough/) [![Crates.io](https://img.shields.io/crates/v/fallthrough)](https://crates.io/crates/fallthrough) [![License](https://img.shields.io/crates/l/fallthrough.svg)](https://github.com/Jules-Bertholet/fallthrough#license)

This crate provides a `fallthrough` macro, which allows performing a pattern match with fallthrough through the arms, in the style of [`C switch`](https://en.cppreference.com/w/c/language/switch).

```rust
use fallthrough::fallthrough;

fn fall(scrutinee: u32) -> u32 {
    let mut ret: u32 = 0;

    fallthrough!(scrutinee,
        val @ (0 | 63..) => ret = val + 7,
        'one: 1 => ret += 8,
        'two: 2 => ret += 9,
        'three: 3 if true => { ret += 10; break 'end },
        'four: 4 => ret = 42,
        'five: 5 => { ret += 1; break 'seven },
        'six: 6 => ret = 3,
        'seven: _ => ret *= 2,
        'end
    );
    ret
}

fn main() {
    assert_eq!(fall(0), 34);
    assert_eq!(fall(1), 27);
    assert_eq!(fall(2), 19);
    assert_eq!(fall(3), 10);
    assert_eq!(fall(4), 86);
    assert_eq!(fall(5), 2);
    assert_eq!(fall(6), 6);
    assert_eq!(fall(7), 0);
    assert_eq!(fall(64), 98);
}
```
