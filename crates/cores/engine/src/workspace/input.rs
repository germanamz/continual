//! The configuration input: what a configuration holds as its source hands it over,
//! each item with the place it is written, before the workspace checks any of it.

use core::time::Duration;
use std::collections::BTreeMap;

use continual_vocabulary::{Located, Location};

use super::step::{CheckEntry, Command};

/// One configuration as written, unchecked. [`Workspace::build`](super::Workspace::build)
/// turns it into a workspace or says what is wrong with it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigInput {
    /// `[continual] jobs`: how many entries may run at once, when the configuration
    /// says. Any whole number the file can hold, so the workspace judges it.
    pub jobs: Option<Located<i64>>,
    /// `[continual] timeout`: how long a step that sets no `timeout` may run.
    pub timeout: Option<Located<Duration>>,
    /// `[steps.<name>]`: each step under the name it is written with.
    pub steps: BTreeMap<String, StepInput>,
}

/// One step as written, unchecked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepInput {
    /// Where the step is defined.
    pub location: Location,
    /// `run`: the command of a tool step, when the step has one.
    pub run: Option<Command>,
    /// `checks`: the check entries of a check step in list order, empty when the step
    /// lists none.
    pub checks: Vec<CheckEntry>,
    /// `timeout`: how long the step may run, when the step says.
    pub timeout: Option<Located<Duration>>,
}
