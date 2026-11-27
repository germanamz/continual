//! The name rule: the text that names a step, a check, a pipeline or a package.

use core::fmt;

/// A name, such as `fmt` or `test-changes`: the lowercase, hyphenated text that names a
/// step, a check, a pipeline or a package.
///
/// Every kind of name follows this one rule, and each core wraps a `Name` in a type of
/// its own, so one kind of name never stands in for another. A name starts with an
/// ASCII lowercase letter and holds only ASCII lowercase letters, digits and hyphens,
/// with every hyphen between two letters or digits. That keeps it safe as a command
/// argument, a configuration key and a file name on every platform. Build one with
/// [`Name::parse`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name {
    text: String,
}

impl Name {
    /// Parses `input` into a name.
    ///
    /// # Errors
    ///
    /// Returns [`NameError::Empty`] when `input` is empty, otherwise the error for the
    /// first character, from the left, that breaks an invariant.
    pub fn parse(input: &str) -> Result<Self, NameError> {
        let mut characters = input.chars().enumerate();
        let Some((_, first)) = characters.next() else {
            return Err(NameError::Empty);
        };
        if !first.is_ascii_lowercase() {
            return Err(NameError::InvalidStart { character: first });
        }
        let mut last = (0, first);
        for (index, character) in characters {
            check_character(index, character, last.1)?;
            last = (index, character);
        }
        if last.1 == '-' {
            return Err(NameError::MisplacedHyphen { index: last.0 });
        }
        Ok(Self {
            text: input.to_owned(),
        })
    }

    /// The name as text, exactly as it was parsed.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.text
    }
}

fn check_character(index: usize, character: char, previous: char) -> Result<(), NameError> {
    if character == '-' && previous == '-' {
        return Err(NameError::MisplacedHyphen { index });
    }
    if character == '-' || character.is_ascii_lowercase() || character.is_ascii_digit() {
        Ok(())
    } else {
        Err(NameError::ForbiddenCharacter { index, character })
    }
}

/// Why a text is not a valid [`Name`].
///
/// Its [`Display`](fmt::Display) output names no kind of name, so whoever reports one
/// says which kind of name the text was meant to be.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NameError {
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

impl fmt::Display for NameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("name is empty"),
            Self::InvalidStart { character } => write!(
                formatter,
                "name starts with {character:?}, not a lowercase letter"
            ),
            Self::ForbiddenCharacter { index, character } => write!(
                formatter,
                "character {index} of the name is {character:?}, not a lowercase letter, digit or hyphen"
            ),
            Self::MisplacedHyphen { index } => write!(
                formatter,
                "the hyphen at character {index} of the name is not between two letters or digits"
            ),
        }
    }
}

impl core::error::Error for NameError {}
