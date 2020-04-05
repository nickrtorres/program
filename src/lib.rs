//! Program provides a single divergent function (`perror`) to emulate C's `perror`

#![warn(clippy::pedantic)]

use std::ffi;
use std::io::{self, Write};
use std::path;
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
    if let Some(cmd) = process_name(std::env::current_exe()) {
        let _ = write!(io::stderr(), "{}: ", cmd.to_string_lossy());
    }

    let _ = writeln!(io::stderr(), "{}", e);
    process::exit(1)
}

/// Attempts to obtain the name of the current process.
fn process_name(process: io::Result<path::PathBuf>) -> Option<ffi::OsString> {
    let cmd = process.ok()?;
    let process = cmd.file_name()?;
    Some(process.to_os_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;
    use std::io::{Error, ErrorKind, Result};
    use std::path::{Path, PathBuf};

    #[test]
    fn it_fails_to_get_process_name_when_current_exe_is_none() {
        let err: Result<PathBuf> = Err(Error::new(ErrorKind::Other, "foobar"));
        assert!(process_name(err).is_none());
    }

    #[test]
    fn it_fails_to_get_process_name_when_filename_is_invalid() {
        let process: Result<PathBuf> = Ok(Path::new("/foo/bar/baz/..").to_path_buf());
        assert!(process_name(process).is_none());
    }

    #[test]
    fn it_succeeds_when_process_name_is_valid() {
        let process: Result<PathBuf> = Ok(Path::new("/foo/bar/baz/qux").to_path_buf());
        assert_eq!(Some(OsString::from("qux")), process_name(process));
    }
}
