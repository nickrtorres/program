Program affords metadata and a divergent method, `perror`, that emulates C's
`perror` from `stdio.h`.

Program is best used alongside [`lazy_static`](https://github.com/rust-lang-nursery/lazy-static.rs).
```rust
use lazy_static::lazy_static;                                                   
use program::Program;

lazy_static! {
    static ref MY_PROGRAM: Program = Program::new("head");
}

fn main() {
    // ...

    if something_bad {
        Program.perror("something bad happened")
    } else if something_really_bad {
        Program.perror(format!("something really bad happened and we need to specify and integer: {}", 42))
    }

    // ...
}
```

