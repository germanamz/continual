//! The workspace: what a repository's `continual.toml` declares, checked as it is built.

mod input;
mod problem;
mod step;
mod step_name;

use core::num::NonZeroUsize;
use core::time::Duration;
use std::collections::BTreeMap;

pub use continual_vocabulary::{Located, Location};
pub use input::{ConfigInput, StepInput};
pub use problem::{Problem, ProblemKind};
pub use step::{CheckEntry, Command, Step, StepKind};
pub use step_name::StepName;

use step::positive_timeout;

/// How long a step may run when neither it nor its configuration sets a `timeout`.
const DEFAULT_TIMEOUT: Duration = Duration::from_mins(30);

/// The steps a repository declares, each one valid, and the job limit it sets.
///
/// Build one with [`Workspace::build`], which rejects a configuration that holds a
/// problem, so a workspace never holds a malformed step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workspace {
    job_limit: Option<NonZeroUsize>,
    steps: Vec<Step>,
}

impl Workspace {
    /// Builds the workspace that `config` declares.
    ///
    /// A step without a `timeout` takes the configuration's, and 30 minutes when the
    /// configuration sets none either.
    ///
    /// # Errors
    ///
    /// Returns every [`Problem`] of the configuration, each at the location `config`
    /// gives the item that is wrong: first `jobs` below 1
    /// ([`ProblemKind::JobLimitBelowOne`]) and a configuration `timeout` of zero
    /// ([`ProblemKind::ZeroTimeout`]), then the steps in the order of their names,
    /// each with its invalid name ([`ProblemKind::InvalidStepName`]), its having both
    /// or neither of `run` and `checks` ([`ProblemKind::BothRunAndChecks`],
    /// [`ProblemKind::NeitherRunNorChecks`]) and its `timeout` of zero.
    pub fn build(config: ConfigInput) -> Result<Self, Vec<Problem>> {
        let job_limit = config.jobs.map(job_limit).transpose();
        let timeout = config.timeout.map(positive_timeout).transpose();
        let default_timeout = match &timeout {
            Ok(Some(timeout)) => *timeout,
            Ok(None) | Err(_) => DEFAULT_TIMEOUT,
        };
        let (steps, step_problems) = build_steps(config.steps, default_timeout);
        match (job_limit, timeout) {
            (Ok(job_limit), Ok(_)) if step_problems.is_empty() => Ok(Self { job_limit, steps }),
            (job_limit, timeout) => {
                let problems = job_limit.err().into_iter().chain(timeout.err());
                Err(problems.chain(step_problems).collect())
            }
        }
    }

    /// The step defined under `name`, or `None` when the workspace has no such step.
    #[must_use]
    pub fn step(&self, name: &str) -> Option<&Step> {
        self.steps.iter().find(|step| step.name().as_str() == name)
    }

    /// How many entries may run at once, or `None` when the configuration sets no
    /// `jobs` and leaves the limit to the caller.
    #[must_use]
    pub fn job_limit(&self) -> Option<NonZeroUsize> {
        self.job_limit
    }
}

/// The job limit that `jobs` sets. A number too large for this platform to count
/// becomes the largest it can count.
fn job_limit(jobs: Located<i64>) -> Result<NonZeroUsize, Problem> {
    let limit = usize::try_from(jobs.value.max(0)).unwrap_or(usize::MAX);
    NonZeroUsize::new(limit).ok_or(Problem {
        location: jobs.location,
        kind: ProblemKind::JobLimitBelowOne { jobs: jobs.value },
    })
}

fn build_steps(
    steps: BTreeMap<String, StepInput>,
    default_timeout: Duration,
) -> (Vec<Step>, Vec<Problem>) {
    let mut built = Vec::new();
    let mut problems = Vec::new();
    for (name, input) in steps {
        match Step::build(name, input, default_timeout) {
            Ok(step) => built.push(step),
            Err(found) => problems.extend(found),
        }
    }
    (built, problems)
}
