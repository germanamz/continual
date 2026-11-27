//! What is wrong with a configuration, and where.

use core::fmt;

use continual_vocabulary::{Location, NameError};

/// One thing wrong with a configuration, at the place the configuration input gave
/// the item that is wrong.
///
/// Its [`Display`](fmt::Display) output is one line, `continual.toml:31: message`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Problem {
    /// Where the item that is wrong is written.
    pub location: Location,
    /// What is wrong with it.
    pub kind: ProblemKind,
}

impl fmt::Display for Problem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { location, kind } = self;
        write!(formatter, "{location}: {kind}")
    }
}

impl core::error::Error for Problem {}

/// What can be wrong with a configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProblemKind {
    /// A step's name breaks the name rule.
    InvalidStepName {
        /// The name as written.
        name: String,
        /// The rule it breaks, at its first offending character.
        error: NameError,
    },
    /// A step has both `run` and `checks`, and a step has exactly one of them.
    BothRunAndChecks {
        /// The step's name as written.
        step: String,
    },
    /// A step has neither `run` nor `checks`, and a step has exactly one of them.
    NeitherRunNorChecks {
        /// The step's name as written.
        step: String,
    },
    /// A `timeout` is zero, a step's own or the configuration's.
    ZeroTimeout,
    /// `jobs` is below 1.
    JobLimitBelowOne {
        /// The number as written.
        jobs: i64,
    },
}

impl fmt::Display for ProblemKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidStepName { name, error } => {
                write!(formatter, "invalid step name {name:?}: {error}")
            }
            Self::BothRunAndChecks { step } => {
                write!(formatter, "step {step:?} has both `run` and `checks`")
            }
            Self::NeitherRunNorChecks { step } => {
                write!(formatter, "step {step:?} has neither `run` nor `checks`")
            }
            Self::ZeroTimeout => {
                formatter.write_str("`timeout` is zero, but must be a positive duration")
            }
            Self::JobLimitBelowOne { jobs } => {
                write!(formatter, "`jobs` is {jobs}, but must be 1 or more")
            }
        }
    }
}
