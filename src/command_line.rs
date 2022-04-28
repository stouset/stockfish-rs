//! Allows access to information about the program's command-line
//! environment.

use std::env;
use std::io::{self, Error, ErrorKind};
use std::path::PathBuf;

use color_eyre::Result;

/// Contains information about the command-line environment the program
/// was invoked in.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandLine {
    /// The directory containing the executable being run.
    pub binary_dir: PathBuf,

    /// The current working directory of the process.
    pub working_dir: PathBuf,
}

impl CommandLine {
    /// Performs global initialization of the command-line interface and
    /// returns an instance with values determined from the environment.
    ///
    /// # Errors
    ///
    /// Returns an error if we failed to perform setup tasks or if we
    /// encountered issues probing the environment.
    pub fn init() -> Result<Self> {
        color_eyre::install()?;

        Ok(Self::new()?)
    }

    /// Returns an instance of `Self`.
    fn new() -> io::Result<Self> {
        Ok(Self {
            binary_dir: Self::binary_dir()?,
            working_dir: Self::working_dir()?,
        })
    }

    /// Returns the absolute path to the binary currently being run.
    ///
    /// # Errors
    ///
    /// Will return a [`std::io::Error`] if unable to determine the
    /// location this program is being run from.
    pub fn binary_path() -> io::Result<PathBuf> {
        let argv0 = env::args_os().next().ok_or_else(|| {
            Error::new(ErrorKind::Other, "unable to locate binary path")
        })?;

        let binary_name: PathBuf = argv0.into();
        let binary_path: PathBuf = binary_name.canonicalize()?;

        Ok(binary_path)
    }

    /// Returns the absolute path to the directory containing the binary
    /// currently being run.
    ///
    /// # Errors
    ///
    /// Will return a [`std::io::Error`] if unable to determine the
    /// location this program is being run from.
    pub fn binary_dir() -> io::Result<PathBuf> {
        let binary_path = Self::binary_path()?;
        let binary_dir = binary_path.parent().ok_or_else(|| {
            Error::new(ErrorKind::Other, "unable to locate binary directory")
        })?;

        Ok(binary_dir.to_path_buf())
    }

    /// Returns the absolute path to the current working directory.
    ///
    /// # Errors
    ///
    /// Will return a `std::io::Error` if the underlying call to
    /// [`std::env::current_dir`] fails.
    pub fn working_dir() -> io::Result<PathBuf> {
        env::current_dir()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_line_new_is_sane() {
        let command_line = CommandLine::new().unwrap();
        let binary_dir = CommandLine::binary_dir().unwrap();
        let working_dir = CommandLine::working_dir().unwrap();

        assert_eq!(command_line.binary_dir, binary_dir);
        assert_eq!(command_line.working_dir, working_dir);
    }

    #[test]
    fn binary_path_is_sane() {
        let binary_path = CommandLine::binary_path().unwrap();

        assert!(binary_path.exists());
        assert!(binary_path.is_absolute());
        assert!(binary_path.is_file());
    }

    #[test]
    fn binary_dir_is_sane() {
        let binary_dir = CommandLine::binary_dir().unwrap();

        assert!(binary_dir.exists());
        assert!(binary_dir.is_absolute());
        assert!(binary_dir.is_dir());
    }

    #[test]
    fn working_dir_is_sane() {
        let working_dir = CommandLine::working_dir().unwrap();

        assert!(working_dir.exists());
        assert!(working_dir.is_absolute());
        assert!(working_dir.is_dir());

        assert_eq!(working_dir, std::env::current_dir().unwrap());
    }
}
