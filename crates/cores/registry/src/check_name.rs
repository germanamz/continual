//! The name that identifies a check in a pipeline.

use core::fmt;
use core::str::FromStr;

/// The name of a check, such as `comments` or `test-changes`: what a contributor
/// types to run that one check, and the label its findings carry.
///
/// A check name starts with an ASCII lowercase letter and holds only ASCII
/// lowercase letters, digits and hyphens, with every hyphen between two letters or
/// digits. That keeps it safe as a command argument, a configuration key and a
/// file name on every platform. Build one with [`CheckName::parse`] or
/// [`str::parse`]; its [`Display`](fmt::Display) output is the text it was parsed
/// from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckName {
    name: String,
}

impl CheckName {
    /// Parses `input` into a check name.
    ///
    /// # Errors
    ///
    /// Returns [`CheckNameError::Empty`] when `input` is empty, otherwise the error
    /// for the first character, from the left, that breaks an invariant.
    pub fn parse(input: &str) -> Result<Self, CheckNameError> {
        let mut characters = input.chars().enumerate();
        let Some((_, first)) = characters.next() else {
            return Err(CheckNameError::Empty);
        };
        if !first.is_ascii_lowercase() {
            return Err(CheckNameError::InvalidStart { character: first });
        }
        let mut last = (0, first);
        for (index, character) in characters {
            check_character(index, character, last.1)?;
            last = (index, character);
        }
        if last.1 == '-' {
            return Err(CheckNameError::MisplacedHyphen { index: last.0 });
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

fn check_character(index: usize, character: char, previous: char) -> Result<(), CheckNameError> {
    if character == '-' && previous == '-' {
        return Err(CheckNameError::MisplacedHyphen { index });
    }
    if character == '-' || character.is_ascii_lowercase() || character.is_ascii_digit() {
        Ok(())
    } else {
        Err(CheckNameError::ForbiddenCharacter { index, character })
    }
}

impl fmt::Display for CheckName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.name)
    }
}

impl FromStr for CheckName {
    type Err = CheckNameError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::parse(input)
    }
}

/// Why a text is not a valid [`CheckName`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckNameError {
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

impl fmt::Display for CheckNameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("check name is empty"),
            Self::InvalidStart { character } => write!(
                formatter,
                "check name starts with {character:?}, not a lowercase letter"
            ),
            Self::ForbiddenCharacter { index, character } => write!(
                formatter,
                "character {index} of the check name is {character:?}, not a lowercase letter, digit or hyphen"
            ),
            Self::MisplacedHyphen { index } => write!(
                formatter,
                "the hyphen at character {index} of the check name is not between two letters or digits"
            ),
        }
    }
}

impl core::error::Error for CheckNameError {}
