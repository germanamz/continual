//! Continual's registry core: the domain model of the check registry and the ports
//! through which it talks to the outside world.
//!
//! The core holds what every check shares, starting with the name of a check, and no
//! technology. It performs no I/O and never reads the clock, the environment or a
//! random source. Running external tools, reading a repository and parsing a
//! language's source live in adapters that implement the core's ports.

mod check_name;

pub use check_name::{CheckName, CheckNameError};
