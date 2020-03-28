//! Program provides a single divergent method `perror` to emulate C's `perror`

/// Store and expose a static, shared reference to a process name.
pub struct Program {
    pub name: &'static str,
}

impl Program {
    /// Create a new `Program` that holds a static, shared reference to a program name.
    pub fn new(name: &'static str) -> Self {
        Program { name }
    }

    /// Print an error message to `stderr` and exit with an exit status of 1.
    pub fn perror<E: std::fmt::Display>(&self, e: E) -> ! {
        eprintln!("{}: {}", self.name, e);
        std::process::exit(1)
    }
}
