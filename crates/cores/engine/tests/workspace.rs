//! Behaviour of [`Workspace`]: the steps a configuration declares, checked as the
//! workspace is built.

use std::num::NonZeroUsize;
use std::time::Duration;

use continual_engine::workspace::ProblemKind::{
    BothRunAndChecks, InvalidStepName, JobLimitBelowOne, NeitherRunNorChecks, ZeroTimeout,
};
use continual_engine::workspace::{
    CheckEntry, Command, ConfigInput, Located, Location, Problem, ProblemKind, Step, StepInput,
    StepKind, StepNameError, Workspace,
};

const THIRTY_MINUTES: Duration = Duration::from_mins(30);

fn line(line: usize) -> Location {
    Location {
        file: "continual.toml".to_owned(),
        line,
    }
}

fn located<T>(value: T, at: usize) -> Located<T> {
    Located {
        value,
        location: line(at),
    }
}

fn problem(at: usize, kind: ProblemKind) -> Problem {
    Problem {
        location: line(at),
        kind,
    }
}

fn cargo(arguments: &[&str]) -> Command {
    Command {
        program: "cargo".to_owned(),
        arguments: arguments.iter().map(|&word| word.to_owned()).collect(),
    }
}

fn checks(names: &[&str]) -> Vec<CheckEntry> {
    let entry = |&name: &&str| CheckEntry {
        name: name.to_owned(),
    };
    names.iter().map(entry).collect()
}

fn tool_step(at: usize) -> StepInput {
    StepInput {
        location: line(at),
        run: Some(cargo(&["fmt"])),
        checks: Vec::new(),
        timeout: None,
    }
}

fn check_step(at: usize) -> StepInput {
    StepInput {
        location: line(at),
        run: None,
        checks: checks(&["comments"]),
        timeout: None,
    }
}

fn config<const COUNT: usize>(steps: [(&str, StepInput); COUNT]) -> ConfigInput {
    let named = |(name, step): (&str, StepInput)| (name.to_owned(), step);
    ConfigInput {
        jobs: None,
        timeout: None,
        steps: steps.into_iter().map(named).collect(),
    }
}

fn built(config: ConfigInput) -> Workspace {
    match Workspace::build(config) {
        Ok(workspace) => workspace,
        Err(problems) => panic!("the configuration should be valid, got: {problems:?}"),
    }
}

fn step<'workspace>(workspace: &'workspace Workspace, name: &str) -> &'workspace Step {
    match workspace.step(name) {
        Some(step) => step,
        None => panic!("the workspace should hold a step named {name:?}"),
    }
}

#[test]
fn a_step_with_run_is_a_tool_step_that_keeps_its_command() {
    let input = StepInput {
        run: Some(cargo(&["fmt", "--all", "--check"])),
        ..tool_step(4)
    };
    let workspace = built(config([("fmt", input)]));
    assert_eq!(
        step(&workspace, "fmt").kind(),
        &StepKind::Tool(cargo(&["fmt", "--all", "--check"]))
    );
}

#[test]
fn a_step_with_checks_is_a_check_step_that_keeps_them_in_list_order() {
    let input = StepInput {
        checks: checks(&["comments", "architecture"]),
        ..check_step(4)
    };
    let workspace = built(config([("conventions", input)]));
    assert_eq!(
        step(&workspace, "conventions").kind(),
        &StepKind::Checks(checks(&["comments", "architecture"]))
    );
}

#[test]
fn a_step_is_found_by_its_name() {
    let workspace = built(config([
        ("fmt", tool_step(4)),
        ("conventions", check_step(7)),
    ]));
    assert_eq!(step(&workspace, "fmt").name().as_str(), "fmt");
    assert_eq!(
        step(&workspace, "conventions").name().as_str(),
        "conventions"
    );
}

#[test]
fn a_name_no_step_has_finds_no_step() {
    let workspace = built(config([("fmt", tool_step(4))]));
    assert_eq!(workspace.step("test"), None);
}

#[test]
fn a_step_with_both_run_and_checks_is_rejected() {
    let input = StepInput {
        checks: checks(&["comments"]),
        ..tool_step(4)
    };
    let step = "lint".to_owned();
    assert_eq!(
        Workspace::build(config([("lint", input)])),
        Err(vec![problem(4, BothRunAndChecks { step })])
    );
}

#[test]
fn a_step_with_neither_run_nor_checks_is_rejected() {
    let input = StepInput {
        run: None,
        ..tool_step(4)
    };
    let step = "lint".to_owned();
    assert_eq!(
        Workspace::build(config([("lint", input)])),
        Err(vec![problem(4, NeitherRunNorChecks { step })])
    );
}

#[test]
fn an_invalid_step_name_is_rejected_with_its_first_offending_character() {
    let kind = InvalidStepName {
        name: "fmt_All".to_owned(),
        error: StepNameError::ForbiddenCharacter {
            index: 3,
            character: '_',
        },
    };
    assert_eq!(
        Workspace::build(config([("fmt_All", tool_step(4))])),
        Err(vec![problem(4, kind)])
    );
}

#[test]
fn a_step_keeps_its_own_timeout() {
    let input = StepInput {
        timeout: Some(located(Duration::from_mins(20), 6)),
        ..tool_step(4)
    };
    let workspace = built(config([("test", input)]));
    assert_eq!(step(&workspace, "test").timeout(), Duration::from_mins(20));
}

#[test]
fn a_step_without_a_timeout_takes_30_minutes() {
    let workspace = built(config([
        ("fmt", tool_step(4)),
        ("conventions", check_step(7)),
    ]));
    assert_eq!(step(&workspace, "fmt").timeout(), THIRTY_MINUTES);
    assert_eq!(step(&workspace, "conventions").timeout(), THIRTY_MINUTES);
}

#[test]
fn a_step_without_a_timeout_takes_the_one_the_workspace_sets() {
    let slow = StepInput {
        timeout: Some(located(Duration::from_secs(90), 9)),
        ..tool_step(7)
    };
    let workspace = built(ConfigInput {
        timeout: Some(located(Duration::from_mins(5), 2)),
        ..config([("fmt", tool_step(4)), ("test", slow)])
    });
    assert_eq!(step(&workspace, "fmt").timeout(), Duration::from_mins(5));
    assert_eq!(step(&workspace, "test").timeout(), Duration::from_secs(90));
}

#[test]
fn a_step_timeout_of_zero_is_rejected_where_it_is_written() {
    let input = StepInput {
        timeout: Some(located(Duration::ZERO, 6)),
        ..tool_step(4)
    };
    assert_eq!(
        Workspace::build(config([("test", input)])),
        Err(vec![problem(6, ZeroTimeout)])
    );
}

#[test]
fn a_workspace_timeout_of_zero_is_rejected_once() {
    let input = ConfigInput {
        timeout: Some(located(Duration::ZERO, 2)),
        ..config([("fmt", tool_step(4)), ("conventions", check_step(7))])
    };
    assert_eq!(Workspace::build(input), Err(vec![problem(2, ZeroTimeout)]));
}

#[test]
fn a_job_limit_of_1_or_more_is_kept() {
    for (jobs, limit) in [(1_i64, 1_usize), (8, 8)] {
        let workspace = built(ConfigInput {
            jobs: Some(located(jobs, 2)),
            ..config([])
        });
        assert_eq!(workspace.job_limit().map(NonZeroUsize::get), Some(limit));
    }
}

#[test]
fn a_job_limit_below_1_is_rejected() {
    for jobs in [0, -1, i64::MIN] {
        let input = ConfigInput {
            jobs: Some(located(jobs, 2)),
            ..config([])
        };
        assert_eq!(
            Workspace::build(input),
            Err(vec![problem(2, JobLimitBelowOne { jobs })])
        );
    }
}

#[test]
fn a_workspace_without_a_job_limit_leaves_it_to_its_caller() {
    assert_eq!(built(config([])).job_limit(), None);
}

#[test]
fn every_problem_of_a_step_is_reported() {
    let input = StepInput {
        run: None,
        timeout: Some(located(Duration::ZERO, 12)),
        ..tool_step(10)
    };
    let error = StepNameError::InvalidStart { character: '-' };
    let (name, step) = ("-".to_owned(), "-".to_owned());
    let expected = vec![
        problem(10, InvalidStepName { name, error }),
        problem(10, NeitherRunNorChecks { step }),
        problem(12, ZeroTimeout),
    ];
    assert_eq!(Workspace::build(config([("-", input)])), Err(expected));
}

#[test]
fn the_problems_of_a_configuration_are_reported_together() {
    let both = StepInput {
        checks: checks(&["comments"]),
        ..tool_step(5)
    };
    let neither = StepInput {
        run: None,
        ..tool_step(9)
    };
    let input = ConfigInput {
        jobs: Some(located(0, 2)),
        timeout: Some(located(Duration::ZERO, 3)),
        ..config([("lint", both), ("fmt", neither)])
    };
    let (fmt, lint) = ("fmt".to_owned(), "lint".to_owned());
    let expected = vec![
        problem(2, JobLimitBelowOne { jobs: 0 }),
        problem(3, ZeroTimeout),
        problem(9, NeitherRunNorChecks { step: fmt }),
        problem(5, BothRunAndChecks { step: lint }),
    ];
    assert_eq!(Workspace::build(input), Err(expected));
}

#[test]
fn a_problem_keeps_the_file_its_location_names() {
    let location = Location {
        file: "apps/web/continual.toml".to_owned(),
        line: 3,
    };
    let input = StepInput {
        location: location.clone(),
        run: None,
        ..tool_step(3)
    };
    let kind = NeitherRunNorChecks {
        step: "lint".to_owned(),
    };
    assert_eq!(
        Workspace::build(config([("lint", input)])),
        Err(vec![Problem { location, kind }])
    );
}

#[test]
fn a_problem_prints_as_its_location_and_what_is_wrong() {
    let step = "lint".to_owned();
    assert_eq!(
        problem(31, BothRunAndChecks { step: step.clone() }).to_string(),
        "continual.toml:31: step \"lint\" has both `run` and `checks`"
    );
    assert_eq!(
        problem(31, NeitherRunNorChecks { step }).to_string(),
        "continual.toml:31: step \"lint\" has neither `run` nor `checks`"
    );
    assert_eq!(
        problem(9, ZeroTimeout).to_string(),
        "continual.toml:9: `timeout` is zero, but must be a positive duration"
    );
    assert_eq!(
        problem(2, JobLimitBelowOne { jobs: -3 }).to_string(),
        "continual.toml:2: `jobs` is -3, but must be 1 or more"
    );
}

#[test]
fn an_invalid_step_name_prints_with_the_name_and_what_breaks_it() {
    let kind = InvalidStepName {
        name: "Fmt".to_owned(),
        error: StepNameError::InvalidStart { character: 'F' },
    };
    assert_eq!(
        problem(4, kind).to_string(),
        "continual.toml:4: invalid step name \"Fmt\": step name starts with 'F', not a lowercase letter"
    );
}

#[test]
fn problems_are_standard_errors() {
    let problem = problem(9, ZeroTimeout);
    let error: &dyn core::error::Error = &problem;
    assert!(error.source().is_none());
}
