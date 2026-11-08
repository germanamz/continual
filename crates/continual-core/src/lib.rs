//! Continual's core: the domain model and the ports through which it talks to the
//! outside world.
//!
//! The core holds every business rule and no technology. It performs no I/O and
//! never reads the clock, the environment or a random source. Checks, findings,
//! pipelines, rules and gates are domain concepts here; running external tools,
//! reading a repository and parsing a language's source live in adapters that
//! implement the core's ports.

mod check_name;

pub use check_name::{CheckName, CheckNameError};
