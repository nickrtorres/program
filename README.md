Program
---
`Program` emulates C's `perror`.

![crates](https://img.shields.io/crates/v/program)

```rust
use program::perror;
use std::io::{self};

fn print_msg(_: &str) -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::Other,
        "something went wrong!",
    ))
}

fn main() {
    if let Err(e) = print_msg("Hello, world!") {
        perror(e);
    }
}
```
```
$ cargo run
   Compiling example v0.1.0 (/root/example)
    Finished dev [unoptimized + debuginfo] target(s) in 0.34s
     Running `target/debug/example`
example: something went wrong!
```

