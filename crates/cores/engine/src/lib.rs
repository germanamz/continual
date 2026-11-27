//! Continual's engine core: what a repository declares in `continual.toml`, as a model
//! that is valid once built.
//!
//! The core holds the rules of the configuration and no technology. It performs no I/O
//! and never reads the clock, the environment or a random source. Reading
//! `continual.toml` and launching tools live in adapters.

pub mod workspace;
