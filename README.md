Program
---
`Program` emulates C's `perror`.

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

