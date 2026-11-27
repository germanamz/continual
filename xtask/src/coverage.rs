//! Coverage thresholds, checked against `cargo llvm-cov`'s JSON summary.

use std::path::Path;

use serde::Deserialize;

/// A core crate, whose lines are held to [`Thresholds::core_lines`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Core {
    name: String,
    directory: String,
}

impl Core {
    /// The core whose package is `name` and whose crate sits in `directory`, relative
    /// to the workspace root. The directory is kept with `/` separators and a trailing
    /// `/`, so it never matches a sibling whose name only starts the same way.
    #[must_use]
    pub fn new(name: &str, directory: &Path) -> Self {
        let parts: Vec<_> = directory
            .components()
            .map(|part| part.as_os_str().to_string_lossy())
            .collect();
        Self {
            name: name.to_owned(),
            directory: format!("{}/", parts.join("/")),
        }
    }
}

/// Minimum coverage, in percent.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Thresholds {
    /// Lines covered across the workspace.
    pub workspace_lines: f64,
    /// Functions covered across the workspace.
    pub workspace_functions: f64,
    /// Lines covered in each core crate.
    pub core_lines: f64,
}

/// The project's thresholds (see `CONVENTIONS.md`).
pub const THRESHOLDS: Thresholds = Thresholds {
    workspace_lines: 90.0,
    workspace_functions: 90.0,
    core_lines: 95.0,
};

#[derive(Deserialize)]
struct Report {
    data: Vec<Export>,
}

#[derive(Deserialize)]
struct Export {
    files: Vec<FileReport>,
    totals: Summary,
}

#[derive(Deserialize)]
struct FileReport {
    filename: String,
    summary: Summary,
}

#[derive(Deserialize)]
struct Summary {
    lines: Counts,
    functions: Counts,
}

#[derive(Deserialize, Clone, Copy, Default)]
struct Counts {
    count: u64,
    covered: u64,
}

impl Counts {
    #[expect(
        clippy::cast_precision_loss,
        reason = "coverage counts are far below 2^52"
    )]
    fn percent(self) -> f64 {
        self.covered as f64 * 100.0 / self.count as f64
    }
}

/// Checks a `cargo llvm-cov --json --summary-only` report against `thresholds` and
/// returns one message per threshold missed. Each core in `cores` has a figure of its
/// own, the sum of the files whose path contains its directory. A core without such
/// files is itself a violation, and so is a workspace without cores.
///
/// # Errors
///
/// Fails when the report is not llvm-cov JSON.
pub fn evaluate(
    report_json: &str,
    cores: &[Core],
    thresholds: &Thresholds,
) -> Result<Vec<String>, serde_json::Error> {
    let report: Report = serde_json::from_str(report_json)?;
    if report.data.is_empty() {
        return Ok(vec!["coverage report holds no data".to_owned()]);
    }
    let no_core = cores
        .is_empty()
        .then(|| "the workspace has no core to hold to the core threshold".to_owned());
    let missed = report
        .data
        .iter()
        .flat_map(|export| export_violations(export, cores, thresholds));
    Ok(no_core.into_iter().chain(missed).collect())
}

fn export_violations(export: &Export, cores: &[Core], thresholds: &Thresholds) -> Vec<String> {
    let mut violations = Vec::new();
    let totals = &export.totals;
    let functions = thresholds.workspace_functions;
    below(
        &mut violations,
        "workspace line coverage",
        totals.lines,
        thresholds.workspace_lines,
    );
    below(
        &mut violations,
        "workspace function coverage",
        totals.functions,
        functions,
    );
    for core in cores {
        let lines = core_lines(&export.files, core);
        if lines.count == 0 {
            violations.push(format!("no coverage data for {}", core.directory));
        } else {
            let what = format!("{} line coverage", core.name);
            below(&mut violations, &what, lines, thresholds.core_lines);
        }
    }
    violations
}

fn core_lines(files: &[FileReport], core: &Core) -> Counts {
    files
        .iter()
        .filter(|file| file.filename.contains(&core.directory))
        .fold(Counts::default(), |sum, file| Counts {
            count: sum.count + file.summary.lines.count,
            covered: sum.covered + file.summary.lines.covered,
        })
}

fn below(violations: &mut Vec<String>, what: &str, counts: Counts, threshold: f64) {
    if counts.count == 0 {
        violations.push(format!("{what} has no data"));
        return;
    }
    let percent = counts.percent();
    if percent < threshold {
        violations.push(format!("{what} {percent:.1}% is below {threshold}%"));
    }
}
