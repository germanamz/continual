//! Continual's vocabulary: the data that several of its cores mean the same way.
//!
//! The crate holds plain records, value objects whose invariant is about the value
//! alone, and no technology. It declares no trait and holds no rule that one core owns,
//! performs no I/O and never reads the clock, the environment or a random source. To a
//! crate that uses it, it is a dependency, as the standard library is.

mod location;
mod name;

pub use location::{Located, Location};
pub use name::{Name, NameError};
