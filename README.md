LibFizzBuzz ![Travis](https://travis-ci.org/alanhoff/rust-libfizzbuzz.svg)
-----------

A Rust library to aid Fizz-Buzz tests.

### Installation

Add this library to your dependencies:

```toml
[dependencies]
libfizzbuzz = "0.1.0"
```

### Usage

```rust
extern crate libfizzbuzz;
use libfizzbuzz as lfb;

// Test if an i64 is a number divisible by 3
let n = lfb::is_fizz(3); // true

// Test if an i64 is a number divisible by 5
let n = lfb::is_buzz(5); // true

// Test if an i64 is a number divisible by 3 and 5
let n = lfb::is_fizzbuzz(15); // true

// Detect if an i64 is divisibre by 3 or 5
let (fizz, buzz, fizzbuzz) = lfb::detect(20); // (false, true, false)
```

### License

Copyright (c) 2015, Alan Hoffmeister <alanhoffmeister@gmail.com>

Permission to use, copy, modify, and distribute this software for any
purpose with or without fee is hereby granted, provided that the above
copyright notice and this permission notice appear in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
