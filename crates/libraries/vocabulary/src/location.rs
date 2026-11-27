//! Where something is written, and a value with the place it is written.

use core::fmt;

/// Where something is written: a file and a line in it.
///
/// Every core means the same by it, whatever is written there: an item of a
/// configuration in `continual.toml`, or what a finding is about in a source file.
///
/// Its [`Display`](fmt::Display) output is `file:line`, such as `continual.toml:31`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location {
    /// The file, as whoever read it names it, such as `continual.toml` or `src/lib.rs`.
    pub file: String,
    /// The line in that file, counting from 1.
    pub line: usize,
}

impl fmt::Display for Location {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { file, line } = self;
        write!(formatter, "{file}:{line}")
    }
}

/// A value with the place it is written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Located<T> {
    /// The value as it is written.
    pub value: T,
    /// Where it is written.
    pub location: Location,
}
