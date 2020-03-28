Program
---
`Program` provides a rust analogue, `perror`, that emulates C's
`perror` from `stdio.h`.

`Program` is best used alongside [`lazy_static`](https://github.com/rust-lang-nursery/lazy-static.rs).
```rust
use lazy_static::lazy_static;                                                   
use program::Program;

lazy_static! {
    static ref MY_PROGRAM: Program = Program::new("head");
}

fn main() {
    // ...

    if something_bad {
        MY_PROGRAM.perror("it can print &str")
    } else if something_really_bad {
        let text = "text";
        MY_PROGRAM.perror(format!("it can print formatted {}", text))
    } else {
        MY_PROGRAM.perror("it can print anything this is std::fmt::Display")
    }
}
```

