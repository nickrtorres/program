//! Program provides a single divergent function (`perror`) to emulate C's `perror`

#![warn(clippy::pedantic)]

use std::ffi;
use std::io::{self, Write};
use std::process;

/// Prints an error message to `stderr` and exit with an exit status of `1`.
///
/// While this function attempts to obtain and write the  name of the current process to `stderr`,
/// write `e` to `stderr`, and exit with an exit status of `1`, only exiting is guranteed.
/// Unfortunately a lot can go wrong when getting the name of the current process.
/// If  for any reason the name of the current process cannot be formed, just print
/// the error given by the user. If the error given by the use cannot be written to stderr just
/// exit with an exit status of 1.
pub fn perror<E: std::fmt::Display>(e: E) -> ! {
    if let Some(cmd) = process_name() {
        let _ = write!(io::stderr(), "{}: ", cmd.to_string_lossy());
    }

    let _ = writeln!(io::stderr(), "{}", e);
    process::exit(1)
}

/// Attempts to obtain the name of the current process.
fn process_name() -> Option<ffi::OsString> {
    let cmd = std::env::current_exe().ok()?;
    let process = cmd.file_name()?;
    Some(process.to_os_string())
}
