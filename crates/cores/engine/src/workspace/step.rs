//! A step: one named definition of what to run, a tool or checks, and for how long.

use core::time::Duration;

use super::input::{Located, StepInput};
use super::problem::{Problem, ProblemKind};
use super::step_name::StepName;

/// A step of a workspace: its name, what it runs and how long it may run.
///
/// A step is a tool step or a check step, never both, and its timeout is never zero.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step {
    name: StepName,
    kind: StepKind,
    timeout: Duration,
}

impl Step {
    /// Builds the step written as `input` under `name`. A step without a `timeout`
    /// takes `default_timeout`.
    ///
    /// # Errors
    ///
    /// Returns every problem of the step: an invalid name, both or neither of `run`
    /// and `checks`, and a `timeout` of zero.
    pub(super) fn build(
        name: String,
        input: StepInput,
        default_timeout: Duration,
    ) -> Result<Self, Vec<Problem>> {
        let location = input.location;
        let kind = step_kind(&name, input.run, input.checks).map_err(|kind| Problem {
            location: location.clone(),
            kind,
        });
        let timeout = input.timeout.map(positive_timeout).transpose();
        let name = StepName::parse(&name).map_err(|error| Problem {
            location: location.clone(),
            kind: ProblemKind::InvalidStepName { name, error },
        });
        match (name, kind, timeout) {
            (Ok(name), Ok(kind), Ok(timeout)) => Ok(Self {
                name,
                kind,
                timeout: timeout.unwrap_or(default_timeout),
            }),
            (name, kind, timeout) => Err(problems_of(name, kind, timeout)),
        }
    }

    /// The name the step is defined under.
    #[must_use]
    pub fn name(&self) -> &StepName {
        &self.name
    }

    /// What the step runs: one command, or checks.
    #[must_use]
    pub fn kind(&self) -> &StepKind {
        &self.kind
    }

    /// How long the step may run: its own `timeout`, or the workspace's default when
    /// it sets none. Never zero.
    #[must_use]
    pub fn timeout(&self) -> Duration {
        self.timeout
    }
}

/// What a step runs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepKind {
    /// A tool step, which runs one command.
    Tool(Command),
    /// A check step, which runs one or more checks in list order.
    Checks(Vec<CheckEntry>),
}

/// The command a tool step runs, already split into words: no shell reads it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    /// The program to launch.
    pub program: String,
    /// The arguments the program gets, in order.
    pub arguments: Vec<String>,
}

/// One check a check step runs. The engine knows no check, so the name is text it
/// passes on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckEntry {
    /// The check's name as written.
    pub name: String,
}

/// The timeout a configuration gives.
///
/// # Errors
///
/// Returns a [`ProblemKind::ZeroTimeout`] problem, at the timeout's location, when it
/// is zero.
pub(super) fn positive_timeout(timeout: Located<Duration>) -> Result<Duration, Problem> {
    if timeout.value.is_zero() {
        return Err(Problem {
            location: timeout.location,
            kind: ProblemKind::ZeroTimeout,
        });
    }
    Ok(timeout.value)
}

fn step_kind(
    name: &str,
    run: Option<Command>,
    checks: Vec<CheckEntry>,
) -> Result<StepKind, ProblemKind> {
    let step = || name.to_owned();
    match (run, checks.is_empty()) {
        (Some(command), true) => Ok(StepKind::Tool(command)),
        (None, false) => Ok(StepKind::Checks(checks)),
        (Some(_), false) => Err(ProblemKind::BothRunAndChecks { step: step() }),
        (None, true) => Err(ProblemKind::NeitherRunNorChecks { step: step() }),
    }
}

fn problems_of(
    name: Result<StepName, Problem>,
    kind: Result<StepKind, Problem>,
    timeout: Result<Option<Duration>, Problem>,
) -> Vec<Problem> {
    let name = name.err().into_iter();
    name.chain(kind.err()).chain(timeout.err()).collect()
}
