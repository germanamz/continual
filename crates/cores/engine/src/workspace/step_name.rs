//! The name a step has in `continual.toml`.

use continual_vocabulary::{Name, NameError};

/// The name of a step, such as `fmt` or `fmt-edited`: the key of its `[steps.<name>]`
/// table, and what a pipeline entry names to run it.
///
/// A step name is a [`Name`] in a type of its own, so it follows the one name rule and
/// no other kind of name stands in for it. Build one with [`StepName::parse`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepName {
    name: Name,
}

impl StepName {
    /// Parses `input` into a step name.
    ///
    /// # Errors
    ///
    /// Returns the [`NameError`] that [`Name::parse`] gives `input`: the rule it breaks,
    /// at its first offending character.
    pub fn parse(input: &str) -> Result<Self, NameError> {
        Name::parse(input).map(|name| Self { name })
    }

    /// The name as text, exactly as it was parsed.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.name.as_str()
    }
}
