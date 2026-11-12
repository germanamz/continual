# Pipeline

The steps a repository declares in `continual.toml`, the pipelines that arrange them into
graphs, how a run walks those graphs, and the events that run pipelines from git hooks and
agent harnesses. A contributor, an agent and CI all run the same pipelines, so a change that
passes on one machine passes everywhere ([pipeline](../VISION.md#pipeline),
[hooks](../VISION.md#hooks)). What a built-in check decides is in
[Built-in checks](built-in-checks.md); how an agent harness calls the events is in
[Harness](harness.md).

## Concepts

Step, required step, pipeline, event, package, check and tool mean what the
[domain glossary](../CONVENTIONS.md#domain-glossary) says. In addition:

- **Root configuration**: the `continual.toml` at the top of a repository. It holds the
  repository's steps and pipelines, and in a repository with several packages it lists them.
- **Package configuration**: the `continual.toml` of one package, which extends the root
  configuration.
- **Tool step**: a step that runs one command (`run`).
- **Check step**: a step that runs one or more built-in checks (`checks`), in list order.
- **Pipeline entry**: a step as one pipeline lists it, with the entries of that pipeline it
  depends on and whether it is required there.
- **Pipeline reference**: how a pipeline names another one in `depends_on`: `ci` for a
  pipeline of the same package, `shared:build` for the `build` pipeline of the package named
  `shared`.
- **Placeholder**: a word in a `run` command, such as `{files}`, that Continual replaces with
  a value from the run before it starts the tool.
- **Run**: one invocation of Continual: the pipelines asked for, the pipelines they depend on,
  and the outcome of each entry.

## Behaviour

### `continual.toml`

> Planned: [Run a repository's steps from continual.toml](../tickets/first-release/1-pipeline/story.md)

A repository with one package needs only the root configuration. Steps say what runs;
pipelines say which steps run, in what order:

```toml
[continual]
version = "0.3.0"

[steps.fmt]
run = "cargo fmt --all --check"
install = "rustup component add rustfmt"

[steps.clippy]
run = "cargo clippy --workspace --all-targets -- -D warnings"

[steps.conventions]

[[steps.conventions.checks]]
check = "comments"

[[steps.conventions.checks]]
check = "architecture"
# the check's settings go here, inline

[steps.test]
run = "cargo nextest run --workspace"
install = "cargo install cargo-nextest --locked"
timeout = "20m"

[pipelines.ci.steps]
fmt = {}
clippy = { depends_on = ["fmt"] }
conventions = { depends_on = ["fmt"], required = false }
test = { depends_on = ["clippy"] }

[pipelines.quick.steps]
fmt = {}
conventions = { depends_on = ["fmt"] }
```

- `[continual] version` pins the Continual release the repository runs. A `continual` binary
  of any other version refuses to run, exits with code 2 and prints the command that installs
  the pinned version, so a contributor's machine and CI always run the same release.
  `--unpinned` skips this check and says so on its first line; it exists for running a build
  of Continual's own working tree ([Checks: self-hosting](checks.md#self-hosting)).
- `[steps.<name>]` defines a step. A step name follows the rules of a check name: lowercase
  ASCII letters, digits and single hyphens between them, starting with a letter.
- `run` makes it a tool step and `checks` makes it a check step; a step has exactly one of
  them. A step never says what it depends on.
- `install` is the command Continual prints when the step's program is not found.
- `timeout` bounds how long the step's tool may run, written as `"90s"`, `"20m"` or `"1h30m"`.
  A step without one gets `[continual] timeout`, which is 30 minutes unless the root
  configuration sets it. No tool runs without a limit ([timeouts](#timeouts)).
- `[pipelines.<name>]` defines a pipeline, named like a step. Its `steps` table lists its
  entries: each key is a step, and each value says which entries of the same pipeline it
  `depends_on` and whether it is `required` (by default it is).
- A pipeline's own `depends_on` lists the pipelines that must pass before it starts. Only the
  pipeline that needs another names it; nothing declares what it blocks.
- Unknown keys are errors, so a misspelt key never silently turns a check off.

Continual reads every configuration before it runs anything. An invalid configuration ends the
run with exit code 2 and reports each problem as a finding, such as
`continual.toml:31: entry "test" of pipeline "ci" depends on "lnt", which the pipeline does not
list`. The problems it reports are an unknown key or value, a pipeline entry naming an unknown
step, an entry depending on one its pipeline does not list, an unknown pipeline or package in
`depends_on`, a cycle among a pipeline's entries or among pipelines (named in order), a step
with both or neither of `run` and `checks`, a timeout that is not a positive duration, and a
version that does not match the binary.

### Commands

A tool step's `run` is split into words the way a shell splits them, with quotes grouping
words, but no shell runs it. There are no pipes, redirections, `&&`, globbing or variable
expansion, so the same step behaves the same on Linux, macOS and Windows. A step that needs a
shell runs a script instead. The tool runs in the directory of the step's package and inherits
Continual's environment.

A step whose program is not on the `PATH` could not run. Continual says which program is
missing and prints the step's `install` command, or says that the step has none.

### Timeouts

> Planned: [Run a repository's steps from continual.toml](../tickets/first-release/1-pipeline/story.md)

A hung tool must never hold up an agent's turn or a CI job until something outside Continual
gives up, so every tool runs with a time limit.

- A step's timeout counts only its own tool's running time, not the time it waits for its
  dependencies.
- When a tool runs past its timeout, Continual stops it and every process it started: first
  it asks them to stop (`SIGTERM` to the process group on Unix, `CTRL_BREAK` to the job on
  Windows), and after 10 seconds it kills whatever is left.
- The step then fails with `timed out after 20m`, and it counts as failed for `required`, so a
  timed-out required entry skips its downstream like any other failure.
- Interrupting Continual (Ctrl-C) stops every running tool the same way, skips the entries not
  started, reports the outcome of every finished one, and exits with code 130.
- Check steps run inside Continual. The `git` commands behind them run with git's prompts
  turned off and a time limit of their own, so a credential prompt or a stuck repository makes
  the check could-not-run instead of hanging ([Built-in checks](built-in-checks.md#design)).

### Placeholders

> Planned: [Run steps on git and agent events](../tickets/first-release/3-events/story.md)

A placeholder is a whole word in `run`. Continual expands it before the tool starts:

| Placeholder | Becomes | Provided by |
| --- | --- | --- |
| `{files}` | One word per file, relative to the step's package directory | an event with files |
| `{message_file}` | The path of the commit message file | the `commit-msg` event |

A step whose `run` uses a placeholder that the current run does not provide is a configuration
error for that run (exit code 2, naming the step and the placeholder), never a silent skip.
So a step that takes `{files}` goes in the pipelines that events run, and the whole-repository
version of the same tool is a separate step:

```toml
[steps.fmt]
run = "cargo fmt --all --check"

[steps.fmt-edited]
run = "rustfmt --edition 2024 --check {files}"
files = ["*.rs"]
```

`files` filters `{files}` with globs relative to the package. When an event has no file that
matches, the step is skipped and reported as skipped, not as passed. Placeholders stay a fixed
set until a use needs more; they are not general variables.

CI inputs reach Continual as the flags `cargo xtask` takes today: `--base`, `--range`,
`--title`, `--body`, and `--ci` with `--labels`. They go to the built-in checks that use them
([Built-in checks](built-in-checks.md)). Tool steps get no placeholder for them until a step
needs one.

### Pipelines

> Planned: [Run a repository's steps from continual.toml](../tickets/first-release/1-pipeline/story.md)

`continual run <pipeline>...` runs the named pipelines, each after the pipelines it depends
on. Inside a pipeline, an entry starts when every entry it depends on has finished, so entries
that do not depend on each other run at the same time, up to `--jobs` at once across the run
(by default, the number of CPUs). Pipelines that do not depend on each other run at the same
time too.

- A step fails when its tool exits with a non-zero code, is killed or runs past its timeout,
  when a check reports findings, or when it could not run.
- When a **required** entry fails, every entry downstream of it in its pipeline is skipped,
  transitively.
- When an entry with `required = false` fails, its failure is reported and the entries that
  depend on it still run.
- A pipeline passes when none of its entries failed. When a pipeline fails, every pipeline
  that depends on it is skipped.
- A step listed in several pipelines of one run runs once for each of them; Continual never
  reuses an outcome.
- The run fails when any entry failed. Skipped entries never pass.

With the configuration above, `continual run ci` with a failing `fmt` skips `clippy`,
`conventions` and `test`. A failing `conventions` is reported, `test` still runs, and the run
fails either way. `continual run quick` runs `fmt` and `conventions` alone: the same steps on a
shorter path.

### Output and exit codes

Continual passes each tool's output through as the tool writes it and judges the tool by its
exit code alone. It never parses, buffers or reinterprets a tool's output, and thresholds such
as a minimum coverage are the tool's own flags.

- A tool's standard output goes to Continual's standard output and its standard error to
  Continual's standard error, line by line as each line ends.
- When a run holds more than one entry, each line carries the step's name as a prefix, such as
  `test | `, and its package in a monorepo, so the lines of steps running at the same time can
  be told apart. A run of a single entry passes the lines through bare.
- When a step fails, Continual prints one line with the reason: the tool's exit code, its
  timeout, or why it could not run, such as `test failed: exit code 101`.
- Check steps print their findings as `path:line: message`
  ([Built-in checks](built-in-checks.md)).
- The last line names every failed entry, in the order they failed, and every entry skipped
  because of them.

| Exit code | Meaning |
| --- | --- |
| 0 | Every entry passed. |
| 1 | At least one entry failed, and every failed entry did run. |
| 2 | The run could not start (invalid configuration, unknown pipeline, wrong version), or a step could not run. |
| 130 | The run was interrupted. |

Continual never caches. Every step runs every time a pipeline asks for it, and tools keep
their own incremental state, such as cargo's `target/` or `tsc --incremental`.

### Packages

> Planned: [Run steps across a repository's packages](../tickets/first-release/5-packages/story.md)

In a monorepo, the root configuration lists the packages, and each package directory holds a
package configuration that extends the root:

```toml
# continual.toml
[continual]
version = "0.3.0"

[workspace]
packages = ["packages/*", "apps/web", "docs"]

[workspace.steps.lint]
run = "eslint ."

[workspace.steps.test]
run = "vitest run"

[workspace.pipelines.check.steps]
lint = {}
test = { depends_on = ["lint"] }

[steps.commit-messages]
[[steps.commit-messages.checks]]
check = "commit-messages"

[pipelines.messages.steps]
commit-messages = {}
```

```toml
# apps/web/continual.toml
name = "web"

[steps.test]
run = "vitest run --coverage"

[pipelines.check]
depends_on = ["shared:build"]

[pipelines.check.steps]
lint = {}
test = { depends_on = ["lint"] }
```

```toml
# docs/continual.toml
name = "docs"

[pipelines.check.steps]
```

- `[workspace] packages` lists globs of directories. Each must hold a `continual.toml` with a
  `name`, unique in the repository.
- `[workspace.steps]` and `[workspace.pipelines]` are shared definitions: every package has
  each of them, run in the package's own directory, and they never run at the root. `[steps]`
  and `[pipelines]` in the root configuration are the root's own and run once, at the root.
- A step or pipeline a package defines under a shared name replaces the shared one whole;
  fields are never merged. Above, `web` keeps the shared `lint`, replaces `test`, and replaces
  `check` to wait for `shared:build`; `docs` replaces `check` with an empty pipeline, so it runs
  neither shared step.
- A pipeline's `depends_on` names `shared:build` to wait for another package's pipeline.
  Continual never infers an edge from `package.json`, `Cargo.toml` or any other manifest.
- Each file belongs to the deepest package that contains it. The root owns every file outside
  the listed packages, so a per-file check in a root step inspects only those, and no file is
  checked twice.

Run from the root, `continual run check` runs `check` in every package that has it and in the
root if the root defines it. Run inside a package directory, it runs that package's `check`.
`continual run web:check` names one package from anywhere. The pipelines they depend on in
other packages run first either way.

### Events

> Planned: [Run steps on git and agent events](../tickets/first-release/3-events/story.md)

An event is a moment at which a repository wants some pipelines to run. `[events]` maps each
one to its pipelines:

```toml
[pipelines.edited.steps]
fmt-edited = {}
conventions = {}

[events]
on_agent_file_edit = ["edited"]
on_agent_turn_end = ["quick"]
on_pre_commit = ["edited"]
on_commit_msg = ["messages"]
on_pre_push = ["ci"]
```

`continual event <event> [--files <path>...]` runs an event. The event names do not depend on
any harness: the [Claude Code mod](harness.md) calls the agent events, and the git hooks that
`continual install git` writes call the git ones. Hook managers such as lefthook or husky can
call `continual event` themselves instead.

| Event | When | Files |
| --- | --- | --- |
| `on_agent_file_edit` | An agent edited files | The edited files |
| `on_agent_turn_end` | An agent is about to end its turn | Every file with uncommitted changes, untracked included |
| `on_pre_commit` | `git commit`, before the message | The staged files |
| `on_commit_msg` | `git commit`, after the message | None; `{message_file}` is provided |
| `on_pre_push` | `git push` | None |

An event runs its pipelines and the pipelines they depend on, like `continual run`. With files,
a package's pipelines run only when the package owns at least one of them, and its `{files}`
holds only its own. The root's pipelines run on every event, since they usually hold
repository checks; a root step that takes `{files}` and owns none of the event's files is
skipped. `on_agent_turn_end` with no uncommitted changes runs nothing and passes.

## Design

### Core

> Planned: [Run a repository's steps from continual.toml](../tickets/first-release/1-pipeline/story.md)

The pipeline's decisions are core code in `continual-core`, in feature modules:

- **`workspace`**: packages, steps, pipelines, their references, and the inheritance from
  `[workspace.steps]` and `[workspace.pipelines]`. Building a `Workspace` resolves every
  reference and rejects unknown steps, pipelines and packages, cycles and malformed steps
  (V3), so a `Workspace` value always holds valid graphs. Step, pipeline and package names are
  value objects with `CheckName`'s parsing rules.
- **`pipeline`**: a `Run` plans the pipelines asked for and the pipelines they depend on, then
  works as a state machine: it says which entries are ready, records each outcome, skips the
  downstream of a failed required entry and the pipelines that depend on a failed one, and ends
  with a verdict naming every failed and skipped entry. It holds no threads and no clock;
  concurrency belongs to the `ToolRunner` adapter.
- **`events`**: maps an event to its pipelines, routes the event's files to the packages that
  contain them, filters them with each step's `files`, and expands placeholders.

The use case loads the workspace, plans the run, and then loops: start every ready tool entry
on the `ToolRunner` up to the job limit, run every ready check entry in the core over the
[check ports](built-in-checks.md#design), pass each output line the runner hands back to
`Report` as it arrives, and record each completion, until nothing is running or ready.

### Ports and adapters

| Port | What the core asks | Adapter |
| --- | --- | --- |
| `ConfigSource` | The root and package configurations, starting from a directory, as core configuration input with each item's location | `crates/adapters/toml-config` |
| `ToolRunner` | Start a command in a directory with a time limit; hand back, as they happen, each line a running command writes (with its stream) and each command's outcome (exit status, killed, timed out), while others keep running; stop every running command on request | `crates/adapters/process` |
| `ChangeHistory` | The staged files and the files with uncommitted changes, for events | `crates/adapters/git` ([Built-in checks](built-in-checks.md#design)) |
| `Report` | Pass a step's output line through, show why a step failed, show the verdict | `crates/adapters/terminal` |

- `toml-config` walks up from the directory it is given to the nearest `continual.toml`, and on
  to the root when that one is a package configuration. It owns the serde DTOs (M1, M4),
  rejects unknown keys and wrong types with their line numbers (V1), and splits `run` into
  words, keeping placeholders as their own tokens. Whether the graphs are valid is decided in
  the core.
- `process` launches tools with `std::process`, one thread per running tool, reads each
  stream line by line as the tool writes it, and reports a program that is not found as its
  own error so the core can print the install command. It enforces each command's time limit
  (A3): it starts every tool in a process group of its own on Unix and a job object on
  Windows, so stopping a tool stops everything the tool started, and it measures the time
  itself, so the core needs no clock. The composition root turns Ctrl-C into a request to stop
  every running command.
- `terminal` writes to the outputs the composition root gives it: standard output and error for
  the CLI, or buffers that the harness adapter returns as the hook's response
  ([Harness](harness.md#design)). It adds the step prefix when the run holds more than one
  entry.

### Composition root

`crates/continual/src/main.rs` reads the current directory, the CPU count and the binary's
version, builds the adapters, and hands them with those values to the core (X1, X4). `cli.rs`
parses `continual run`, `continual event` and `continual install git`, calls one core operation
each, and maps the verdict to the exit code (A2).

## Decisions

- **Pipelines are declared apart from steps.** Steps say what runs; pipelines say which steps
  run, in what order and what each waits for. One step serves several execution paths, such as
  a full `ci` and a `quick` run before an agent's turn ends, and a package leaves out a shared
  step by its pipelines alone. A pipeline names the pipelines it depends on, and nothing
  declares what it blocks, so every edge is written where it is used. Rejected: dependencies
  declared on the steps themselves, which tie each step to one order; a fixed ordered list,
  which serialises independent work and cannot describe a monorepo; and edges inferred from
  package manifests, as turbo's `^build` does, which hides why a step runs.
- **A failed required entry skips its downstream; entries are required by default.** A
  repository with no settings gets the strict behaviour. `required` belongs to the pipeline
  entry, so the same step can stop one pipeline and only report in another. Rejected: a
  setting on the step, and one switch for the whole pipeline, which cannot keep the steps a
  pipeline depends on stopping while the others continue.
- **Tools' output streams through, judged by exit code.** Continual augments the repository's
  tools and never replaces them: it passes their logs on as they are written, so a long test
  run shows its progress, and it reports each failure by its exit code. A step-name prefix
  keeps lines of steps running at the same time readable. Rejected: capturing each step's
  output and printing it whole when the step finishes, which hides a long step's progress;
  streaming parallel steps without a prefix, which mixes their lines; and parsing each tool's
  output into findings or thresholds, which would reimplement every tool's reporting.
- **No caching.** Continual is an orchestrator, not a build system, and the tools already keep
  their own incremental state. A step listed in two pipelines of one run runs twice. Rejected:
  turbo- or nx-style output and remote caches, which would make results depend on cache state.
- **No shell.** Commands are split into words and launched directly, so a step runs the same
  way on every platform. Rejected: `sh -c` and `cmd /c`, which differ by platform and make
  placeholder quoting unsafe.
- **A placeholder the run cannot fill is an error, and file steps are their own steps.** One
  meaning per step keeps every run explicit. Rejected: expanding `{files}` to every tracked file
  on a full run, which can exceed the platform's command-line limit; and a second command per
  step for file runs, which gives one step two behaviours.
- **CI inputs start as the flags `cargo xtask` takes.** The built-in checks get exactly the
  inputs the repository's own checks use today, and the set grows as Continual is built.
  Rejected: designing placeholders for CI inputs before a tool step needs one.
- **Every package inherits every shared step and pipeline.** A package's configuration holds
  only what makes it different, and what the package redefines replaces the shared definition
  whole. Rejected: an `extends` list naming what each package takes, which repeats the same
  list in every package; and merging fields, which makes a definition hard to read.
- **The release is pinned in `continual.toml`.** The same file that declares the pipelines says
  which Continual runs them, so a local run and CI cannot drift apart. Rejected: pinning only in
  CI.
- **The scheduler is a core state machine; concurrency is the adapter's.** The core stays free
  of threads (B6) and the scheduling rules are tested with a fake `ToolRunner`.
- **Every tool runs with a time limit, enforced by the process adapter.** The limit is the
  repository's configuration and the core passes it with each command; measuring time and
  stopping processes is machinery (A3, X1). A timed-out step is an ordinary failure, so
  `required` decides what it skips. Rejected: no limit unless configured, which leaves a hung
  tool blocking an agent's turn or a CI job until the CI's own timeout; a single limit for the
  whole run, which cannot say which step hung; a way to turn the limit off, since a long step
  sets a long timeout instead; and timing in the core, which would need a clock port.
- **Stopping a tool stops its whole process tree.** Test runners and build tools start
  processes of their own, and killing only the parent leaves those running. Rejected: killing
  the direct child only.
