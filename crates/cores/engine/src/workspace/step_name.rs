//! The name a step has in `continual.toml`.

use core::fmt;

/// The name of a step, such as `fmt` or `fmt-edited`: the key of its `[steps.<name>]`
/// table, and what a pipeline entry names to run it.
///
/// A step name follows the rules a check name follows. It starts with an ASCII
/// lowercase letter and holds only ASCII lowercase letters, digits and hyphens, with
/// every hyphen between two letters or digits. Build one with [`StepName::parse`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepName {
    name: String,
}

impl StepName {
    /// Parses `input` into a step name.
    ///
    /// # Errors
    ///
    /// Returns [`StepNameError::Empty`] when `input` is empty, otherwise the error
    /// for the first character, from the left, that breaks an invariant.
    pub fn parse(input: &str) -> Result<Self, StepNameError> {
        let mut characters = input.chars().enumerate();
        let Some((_, first)) = characters.next() else {
            return Err(StepNameError::Empty);
        };
        if !first.is_ascii_lowercase() {
            return Err(StepNameError::InvalidStart { character: first });
        }
        let mut last = (0, first);
        for (index, character) in characters {
            check_character(index, character, last.1)?;
            last = (index, character);
        }
        if last.1 == '-' {
            return Err(StepNameError::MisplacedHyphen { index: last.0 });
        }
        Ok(Self {
            name: input.to_owned(),
        })
    }

    /// The name as text, exactly as it was parsed.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.name
    }
}

fn check_character(index: usize, character: char, previous: char) -> Result<(), StepNameError> {
    if character == '-' && previous == '-' {
        return Err(StepNameError::MisplacedHyphen { index });
    }
    if character == '-' || character.is_ascii_lowercase() || character.is_ascii_digit() {
        Ok(())
    } else {
        Err(StepNameError::ForbiddenCharacter { index, character })
    }
}

/// Why a text is not a valid [`StepName`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepNameError {
    /// The text is empty.
    Empty,
    /// The first character is not an ASCII lowercase letter.
    InvalidStart {
        /// The first character.
        character: char,
    },
    /// A character is not an ASCII lowercase letter, digit or hyphen.
    ForbiddenCharacter {
        /// Position of the character, counting characters (not bytes) from 0.
        index: usize,
        /// The forbidden character.
        character: char,
    },
    /// A hyphen follows another hyphen or ends the name.
    MisplacedHyphen {
        /// Position of the hyphen, counting characters (not bytes) from 0.
        index: usize,
    },
}

impl fmt::Display for StepNameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("step name is empty"),
            Self::InvalidStart { character } => write!(
                formatter,
                "step name starts with {character:?}, not a lowercase letter"
            ),
            Self::ForbiddenCharacter { index, character } => write!(
                formatter,
                "character {index} of the step name is {character:?}, not a lowercase letter, digit or hyphen"
            ),
            Self::MisplacedHyphen { index } => write!(
                formatter,
                "the hyphen at character {index} of the step name is not between two letters or digits"
            ),
        }
    }
}

impl core::error::Error for StepNameError {}
