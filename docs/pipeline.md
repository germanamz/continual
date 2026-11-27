# Pipeline

The steps a repository declares in `continual.toml`, the pipelines that arrange them into
graphs, how a run walks those graphs, and the events that run pipelines from git hooks and
agent harnesses. A contributor, an agent and CI all run the same pipelines, so a change that
passes on one machine passes everywhere ([pipeline](../VISION.md#pipeline),
[hooks](../VISION.md#hooks)). What a built-in check decides is in
[Built-in checks](built-in-checks.md); how an agent harness calls the events is in
[Harness](harness.md).

## Concepts

Step, required entry, pipeline, event, package, check and tool mean what the
[domain glossary](../CONVENTIONS.md#domain-glossary) says. In addition:

- **Root configuration**: the `continual.toml` at the top of a repository. It holds the
  repository's steps and pipelines, and in a repository with several packages it lists them.
- **Package configuration**: the `continual.toml` of one package, which extends the root
  configuration.
- **Tool step**: a step that runs one command (`run`).
- **Check step**: a step that runs one or more built-in checks (`checks`), in list order.
- **Pipeline entry**: a step as one pipeline lists it, in the pipeline's ordered list of
  entries, with the entries above it that it depends on and whether it is required there.
- **Starting entry**: the first entry of a pipeline, the one that starts it.
- **Pipeline reference**: how a pipeline names another one in `depends_on`: `ci` for a
  pipeline of the same package, `shared:build` for the `build` pipeline of the package named
  `shared`.
- **Placeholder**: a word in a `run` command, such as `{files}`, that Continual replaces with
  a value from the run before it starts the tool.
- **Run**: one invocation of Continual: the pipelines asked for, the pipelines they depend on,
  and the outcome of each entry.

## Behaviour

### `continual.toml`

> Planned: [Run a repository's steps from continual.toml](../tickets/first-release/2-pipeline/story.md)

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

[[pipelines.ci.steps]]
step = "fmt"

[[pipelines.ci.steps]]
step = "clippy"
depends_on = ["fmt"]

[[pipelines.ci.steps]]
step = "conventions"
depends_on = ["fmt"]
required = false

[[pipelines.ci.steps]]
step = "test"
depends_on = ["clippy"]

[[pipelines.quick.steps]]
step = "fmt"

[[pipelines.quick.steps]]
step = "conventions"
depends_on = ["fmt"]
```

- `[continual] version` pins the Continual release the repository runs. A `continual` binary
  of any other version refuses to run, exits with code 2 and prints the command that installs
  the pinned version, so a contributor's machine and CI always run the same release.
  `--unpinned` skips this check and says so on its first line; it exists for running a build
  of Continual's own working tree ([Checks: self-hosting](checks.md#self-hosting)). The
  version is checked before anything else in the file, and a binary of another version reports
  only the mismatch, since the rest of the file may hold keys that binary does not know.
- `[continual] jobs` is how many entries may run at once across a run, check steps and tool
  steps alike. It is a whole number of 1 or more, and only the root configuration holds it.
  Without it the limit is the number of CPUs, and `jobs = 1` runs one entry at a time
  ([pipelines](#pipelines)).
- `[steps.<name>]` defines a step. A step name follows the rules of a check name: lowercase
  ASCII letters, digits and single hyphens between them, starting with a letter.
- `run` makes it a tool step and `checks` makes it a check step; a step has exactly one of
  them. A step never says what it depends on.
- `install` is the command Continual prints when the step's program is not found.
- `timeout` bounds how long the step may run, a tool step's tool or a check step's checks,
  written as `"90s"`, `"20m"` or `"1h30m"`. A step without one gets `[continual] timeout`,
  which is 30 minutes unless the root configuration sets it. No step runs without a limit
  ([timeouts](#timeouts)).
- `batch = true` lets a step whose `{files}` do not fit one command line run its tool once for
  each batch of them ([placeholders](#placeholders)).
- `[pipelines.<name>]` defines a pipeline, named like a step. Its entries are an array,
  `[[pipelines.<name>.steps]]`, which TOML keeps in the order written. Each entry names its
  `step`, the entries above it that it `depends_on`, and whether it is `required` (by default
  it is). A step appears at most once in a pipeline.
- The first entry is the starting entry: it has no `depends_on` and starts the pipeline. Every
  later entry depends on at least one entry above it and never on one below, so the written
  order is always an order the pipeline can run in, and a cycle cannot be written.
- A pipeline's own `depends_on` lists the pipelines that must pass before its starting entry
  starts. Only the pipeline that needs another names it; nothing declares what it blocks.
- A pipeline with no entries, `steps = []`, runs nothing and passes.
- Unknown keys are errors, so a misspelt key never silently turns a check off.

Continual reads every configuration before it runs anything. An invalid configuration ends the
run with exit code 2 and reports each problem as a finding, such as
`continual.toml:31: entry "test" of pipeline "ci" depends on "lnt", which is not listed above
it`. The problems it reports are an unknown key or value, a pipeline entry naming an unknown
step, a step listed twice in one pipeline, a starting entry with `depends_on`, a later entry
without it, an entry depending on one that is not above it, an unknown pipeline or package in
a pipeline's `depends_on`, a cycle among pipelines (named in order), a step with both or
neither of `run` and `checks`, an unclosed quote or a shell operator in a step's `run`,
`batch` on a step whose `run` has no `{files}`, a timeout that is not a positive duration,
and a `jobs` below 1. A version that does not match the binary is
reported alone, before any of these.

### Commands

> Planned: [Run a repository's steps from continual.toml](../tickets/first-release/2-pipeline/story.md)

A tool step's `run` is split into words the way a shell splits them, with quotes grouping
words, but no shell runs it. There are no pipes, redirections, `&&`, globbing or variable
expansion, so the same step behaves the same on Linux, macOS and Windows. A step that needs a
shell runs a script instead. A shell operator outside quotes, such as `&&`, `|` or `>`, is a
configuration error that names it, so it never reaches the tool as an argument. The tool runs in the directory of the step's package and inherits
Continual's environment.

A step whose program is not on the `PATH` could not run. Continual says which program is
missing and prints the step's `install` command, or says that the step has none.

### Timeouts

> Planned: [Run a repository's steps from continual.toml](../tickets/first-release/2-pipeline/story.md)

A hung step must never hold up an agent's turn or a CI job until something outside Continual
gives up, so every step runs with a time limit, a tool step and a check step alike.

- A step's timeout counts only its own running time, not the time it waits for its
  dependencies.
- When a tool runs past its timeout, Continual stops it and every process it started: first
  it asks them to stop (`SIGTERM` to the tool's process group on Unix, `CTRL_BREAK` to it on
  Windows), and after 10 seconds it kills whatever is left (the process group on Unix, the
  tool's job object on Windows).
- The step then fails with `timed out after 20m`, and it counts as failed for `required`, so a
  timed-out required entry skips its downstream like any other failure.
- Interrupting Continual (Ctrl-C) stops every running tool the same way, skips the entries not
  started, reports the outcome of every finished one, and exits with code 130. So does telling
  it to stop (`SIGTERM` or `SIGHUP` on Unix, closing its console on Windows), as a cancelled CI
  job or a harness that gives up on a hook does, so no tool outlives Continual.
- A check step runs inside Continual, so there is no process to stop. When it runs past its
  timeout, Continual stops waiting for its checks and the step fails with
  `timed out after 20m`, like a tool step.
- The `git` commands behind a check also run with git's prompts turned off and a limit of
  their own, so a credential prompt or a stuck repository makes the check could-not-run well
  before the step's timeout ([Built-in checks](built-in-checks.md#design)).
- An agent harness applies a timeout of its own to each hook, which
  `continual install claude` writes from these limits. After changing a timeout, run the
  installer again ([Harness](harness.md#keeping-the-mod-up-to-date)).

### Placeholders

> Planned: [Run steps on git and agent events](../tickets/first-release/4-events/story.md)

A placeholder is a whole word in `run`. Continual expands it before the tool starts:

| Placeholder | Becomes | Provided by |
| --- | --- | --- |
| `{files}` | One word per file, relative to the step's package directory; a path that starts with `-` gets a `./` in front, so no tool reads it as an option | an event with files |
| `{message_file}` | The path of the commit message file | the `on_commit_msg` event |

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
matches, the step has nothing to do: it is skipped and reported as skipped, not as passed.
That is not a failure. The entries that depend on it still run, and it does not fail the run.
Placeholders stay a fixed set until a use needs more; they are not general variables.

Every platform limits how long a command line may be. A step whose `{files}` do not fit that
limit could not run: Continual says how many files the step held and names `batch`. Splitting
the files is the last resort, and a step asks for it itself with `batch = true`. Continual
then runs the tool once for each batch of files that fits, one batch after another, as
`xargs` does. Every batch runs, even after one fails; the step fails when any of them does,
and its timeout covers all of them together. A step whose files fit runs its tool once, with
or without `batch`.

CI inputs reach Continual as the flags `cargo xtask` takes today: `--base`, `--range`,
`--title`, `--body`, and `--ci` with `--labels`. They go to the built-in checks that use them
([Built-in checks](built-in-checks.md)). Tool steps get no placeholder for them until a step
needs one.

### Pipelines

> Planned: [Run a repository's steps from continual.toml](../tickets/first-release/2-pipeline/story.md)

`continual run <pipeline>...` runs the named pipelines, each after the pipelines it depends
on. A pipeline starts with its starting entry; every later entry starts when every entry it
depends on has finished, so entries that do not depend on each other run at the same time, up
to the job limit across the run: `[continual] jobs`, or `--jobs <n>` for one invocation, and
the number of CPUs when neither is given. A check step counts as one job, like a tool step.
With a limit of 1, entries run one after another. Pipelines that do not depend on each other
run at the same time too.

When more entries are ready than there are free jobs, the order is fixed, so every machine
schedules the same way: the pipelines a pipeline depends on come before it, in the order its
`depends_on` lists them, the pipelines asked for come in the order the command names them, a
pipeline that runs in several packages takes them in the order of their names, the root last,
and within a pipeline the entry written first starts first.

- A step fails when its tool exits with a non-zero code or is killed, when a check reports
  findings, when the step runs past its timeout, or when it could not run.
- When a **required** entry fails, every entry downstream of it in its pipeline is skipped,
  transitively.
- When an entry with `required = false` fails, its failure is reported and the entries that
  depend on it still run.
- A pipeline passes when none of its entries failed. When a pipeline fails, every pipeline
  that depends on it is skipped.
- A step listed in several pipelines of one run runs once for each of them; Continual never
  reuses an outcome.
- An entry that an [event](#events) leaves with no matching files is skipped without failing:
  it counts as finished, so the entries that depend on it still run.
- The run fails when any entry failed. A skipped entry is never reported as passed.

With the configuration above, `continual run ci` starts with `fmt`. When it passes, `clippy`
and `conventions` start together, and `test` follows `clippy`. A failing `fmt` skips
`clippy`, `conventions` and `test`. A failing `conventions` is reported, `test` still runs, and
the run fails either way. `continual run quick` runs `fmt` and then `conventions`: the same
steps on a shorter path.

### Output and exit codes

> Planned: [Run a repository's steps from continual.toml](../tickets/first-release/2-pipeline/story.md)

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
| 0 | No entry failed: each one passed, or was skipped for having no matching files. |
| 1 | At least one entry failed, and every failed entry did run. |
| 2 | The run could not start (invalid configuration, unknown pipeline, wrong version), or a step could not run. |
| 130 | The run was interrupted or told to stop. |

Continual never caches. Every step runs every time a pipeline asks for it, and tools keep
their own incremental state, such as cargo's `target/` or `tsc --incremental`.

### Packages

> Planned: [Run steps across a repository's packages](../tickets/first-release/6-packages/story.md)

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

[[workspace.pipelines.check.steps]]
step = "lint"

[[workspace.pipelines.check.steps]]
step = "test"
depends_on = ["lint"]

[steps.commit-messages]
[[steps.commit-messages.checks]]
check = "commit-messages"

[[pipelines.messages.steps]]
step = "commit-messages"
```

```toml
# apps/web/continual.toml
name = "web"

[steps.test]
run = "vitest run --coverage"

[pipelines.check]
depends_on = ["shared:build"]

[[pipelines.check.steps]]
step = "lint"

[[pipelines.check.steps]]
step = "test"
depends_on = ["lint"]
```

```toml
# docs/continual.toml
name = "docs"

[pipelines.check]
steps = []
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

> Planned: [Run steps on git and agent events](../tickets/first-release/4-events/story.md)

An event is a moment at which a repository wants some pipelines to run. `[events]` maps each
one to its pipelines:

```toml
[[pipelines.edited.steps]]
step = "fmt-edited"

[[pipelines.edited.steps]]
step = "conventions"
depends_on = ["fmt-edited"]

[events]
on_agent_file_edit = ["edited"]
on_agent_turn_end = ["quick"]
on_pre_commit = ["edited"]
on_commit_msg = ["messages"]
on_pre_push = ["ci"]
```

`continual event <event> [--files <path>...] [--message-file <path>]` runs an event.
`--files` gives `on_agent_file_edit` its files and `--message-file` gives `on_commit_msg` its
message. The event names do not depend on any harness: the [Claude Code mod](harness.md) calls
the agent events, and the git hooks that `continual install git` writes call the git ones.
Hook managers such as lefthook or husky can call `continual event` themselves instead. A
repository that uses the Claude Code mod runs `continual install claude` again after it
changes what an agent event runs ([Harness](harness.md#keeping-the-mod-up-to-date)).

| Event | When | Files |
| --- | --- | --- |
| `on_agent_file_edit` | An agent edited files | The edited files |
| `on_agent_turn_end` | An agent is about to end its turn | Every file with uncommitted changes, untracked included |
| `on_pre_commit` | `git commit`, before the message | The staged files |
| `on_commit_msg` | `git commit`, after the message | None; `{message_file}` is provided |
| `on_pre_push` | `git push` | None |

An event runs its pipelines and the pipelines they depend on, like `continual run`. An event
without files runs them exactly as `continual run` does: nothing is filtered or skipped, and a
per-file check inspects every file of its package. With files, a package's pipelines run only
when the package owns at least one of them, and its `{files}` holds only its own. The root's
pipelines run on every event, since they usually hold repository checks; a root step that
takes `{files}` and owns none of the event's files is skipped. `on_agent_turn_end` with no
uncommitted changes runs nothing and passes.

A deleted file is never among an event's files, since no tool can read it, and a renamed file
appears under its new name.

## Design

### The engine core

> Planned: [Run a repository's steps from continual.toml](../tickets/first-release/2-pipeline/story.md)

The pipeline's decisions are code in the engine core, `continual-engine`, one of Continual's
cores ([Workspace](workspace.md#planned-crates)), in feature modules:

- **`workspace`**: packages, steps, pipelines, their references, and the inheritance from
  `[workspace.steps]` and `[workspace.pipelines]`. Building a `Workspace` resolves every
  reference and rejects unknown steps, pipelines and packages, entries that break the
  starting-entry and above-only rules, cycles among pipelines and malformed steps (V3), so a
  `Workspace` value always holds valid graphs, each with its entries in written order. The
  use case that loads it asks `CheckRunner` to read each check entry's settings and passes
  the answers in (M5), so an unknown check and a setting its check rejects are caught here
  too ([Built-in checks](built-in-checks.md#the-registry-core)). It keeps `[review_gate]` and
  `[[blocked_commands]]` as located values that only the gate interprets
  ([Harness](harness.md#design)). Step, pipeline and package names are value objects of the
  engine's own, each a type of its own over the one name rule that a check name follows too.
  The engine writes that rule itself until it moves to the library crate
  ([Workspace](workspace.md#library-crates)).

  A `Workspace` is built from configuration input: plain records that hold each item as
  written, with its location, a file and a line. A location and a located value mean the
  same in every core, so those two records, `Location` and `Located<T>`, are the library
  crate's and not the engine's own. The steps arrive by name, so no name can be given
  twice, a tool step's `run` as a program and its arguments, and a check step's entries in
  list order, where a step that lists none has no `checks`. Building reports every problem
  at once instead of stopping at the first (E9): `jobs` and the configuration's `timeout`
  first, then the steps in the order of their names. Each problem holds what is wrong and
  the location of the item that is wrong, and prints as `continual.toml:31: message`.
- **`pipeline`**: a `Run` plans the pipelines asked for and the pipelines they depend on, then
  works as a state machine: it says which entries are ready, in the fixed order the
  [pipelines](#pipelines) section gives, records each outcome, skips the
  downstream of a failed required entry and the pipelines that depend on a failed one, and ends
  with a verdict naming every failed and skipped entry. It holds no threads and no clock: it
  says what may start and records what finished.
- **`events`**: maps an event to its pipelines, routes the event's files to the packages that
  contain them, filters them with each step's `files`, and expands placeholders. It measures
  each expanded command against the platform's command-line limit, a value the composition
  root passes in: a command that does not fit makes its step could-not-run, or is split into
  batches when the step sets `batch`.

The use case loads the workspace, plans the run, and then drives it as futures. Each entry it
starts is one future: a tool entry's comes from `ToolRunner` and a check entry's from
`CheckRunner`. The use case keeps as many of them running as the job limit allows, started in
the order the `Run` gives, passes each output line to `Report` as it arrives, records each
outcome when its future ends and starts what that makes ready, until nothing is running or
ready. Each path from a pipeline's starting entry to its last one therefore advances on its
own, and a check entry holds up neither the lines and outcomes of the tools running beside it
nor a request to stop. A check entry's future is raced against its step's time limit, a
future the `Timer` port hands over: when the limit ends first, the use case drops the entry's
future and records the step as timed out. The engine knows no check: a check entry is a name
and settings that it passes on.

The engine uses the language's own futures and nothing else. It holds no thread and depends
on no runtime (B1, B6): the composition root builds the runtime that executes a run's
futures, and the adapter behind each port decides how its future does its work.

The run's inputs are a small record of the engine's: the base revision, the commit range, the
pull request's title and body, and whether the run is a CI run and with which labels
([placeholders](#placeholders)). Every check step carries it to `CheckRunner`. `--base` is
`origin/main` unless given, a default the command line holds once for every command, so the
cores always receive a base revision.

The engine also answers what two other cores ask about the configuration, each through a
bridge:

- For the check registry, the entries one check has in the configuration at the merge base
  with a revision ([Built-in checks](built-in-checks.md#ports-and-adapters)). This is an
  operation of its own that needs only `ChangeHistory` and `ConfigSource`. It reads every
  `continual.toml` at that revision into configuration input and picks the check's entries
  out of it, each with its package. It builds no `Workspace`, asks no `CheckRunner` and
  ignores the version the base pins, since the base only lends its settings. A base
  configuration that cannot be read is an error, and the check that asked could not run.
- For the gate, its sections of the configuration ([Harness](harness.md#design)).

### Ports and adapters

> Planned: [Run a repository's steps from continual.toml](../tickets/first-release/2-pipeline/story.md)

> Planned: [Run steps on git and agent events](../tickets/first-release/4-events/story.md)

| Port | What the engine asks | Adapter |
| --- | --- | --- |
| `ConfigSource` | The root and package configurations, starting from a directory, as configuration input with each item's location, a check entry's settings among them as located values left for its check to read; and the same input from one configuration's text, for the guard's base revision ([Built-in checks](built-in-checks.md#guard)) | `crates/adapters/toml-config` |
| `ToolRunner` | Run a command in a directory with a time limit, or a batched step's commands one after another under one limit, as a future that hands over each line a command writes as the line ends (with its stream) and ends with the outcome (exit status, killed, timed out), while other commands keep running; stop every running command on request | `crates/adapters/process` |
| `CheckRunner` | Read a check entry's settings, which are accepted or rejected with their lines; run a step's check entries in a package, leaving out the directories of the packages inside it, over the event's files when it carries any and with the run's inputs, as a future that ends with the step's finding lines and its outcome | `crates/adapters/engine-registry`, a bridge to the check registry ([Built-in checks](built-in-checks.md#bridges)) |
| `Timer` | A future that ends once a duration has passed, which the engine races a check step against | `crates/adapters/context`, the execution context ([Built-in checks](built-in-checks.md#ports-and-adapters)) |
| `ChangeHistory` | The staged files and the files with uncommitted changes, for events; and each `continual.toml` at the merge base with a revision, with its text, for the configuration at the base revision | `crates/adapters/git` ([Built-in checks](built-in-checks.md#ports-and-adapters)) |
| `Report` | Pass a step's output line through, show why a step failed, show the verdict | `crates/adapters/terminal` |

- `toml-config` walks up from the directory it is given to the nearest `continual.toml`, and on
  to the root when that one is a package configuration. It owns the serde DTOs (M1, M4),
  rejects unknown keys and wrong types with their line numbers (V1), and splits `run` into
  words with the `shell-words` tokenizer ([Workspace](workspace.md#planned-crates)), keeping
  placeholders as their own tokens and rejecting an operator. A check entry is the exception: the
  adapter reads its `check` name and passes the settings beside it on as located values,
  since only the check knows its own settings
  ([Built-in checks](built-in-checks.md#bridges)). It checks the shape of `[review_gate]`
  and `[[blocked_commands]]` like the rest of the file, the names of the gated actions
  included, so every command rejects a mistake in them, and passes them on as located
  values too. Whether the graphs are valid is decided in the engine.
- `process` launches tools with `std::process`, one thread per running tool, reads each
  stream line by line as the tool writes it, ends the command's future when the tool does, and
  reports a program that is not found as its own error so the engine can print the install
  command. It enforces each command's time limit
  (A3): it starts every tool in a process group of its own, and on Windows in a job object as
  well, so stopping a tool stops everything the tool started, and it measures the time
  itself, so the engine needs no clock. The composition root turns Ctrl-C, and a signal that
  tells Continual to stop, into a request to stop every running command.
- `terminal` writes to the outputs the composition root gives it: standard output and error for
  the CLI, or buffers that the harness adapter returns as the hook's response
  ([Harness](harness.md#design)). It adds the step prefix when the run holds more than one
  entry.

### Composition root

> Planned: [Run a repository's steps from continual.toml](../tickets/first-release/2-pipeline/story.md)

> Planned: [Run steps on git and agent events](../tickets/first-release/4-events/story.md)

`crates/continual/src/main.rs` reads the current directory, the CPU count, the platform's
command-line limit and the binary's version and builds the adapters and the cores, each after
what it calls (B5): the technology adapters, the engine's reader of the base revision's
entries, each check core with its bridge, the check registry, and last the engine's use
cases, which get their adapters and those values (X1, X4). The two adapters of the
`engine-registry` bridge sit between the engine and the registry, one for each direction. It
also builds the runtime that executes a run's futures. `cli.rs` parses `continual run`,
`continual event` and `continual install git`, calls one engine operation each, waits for it
on that runtime, and maps the verdict to the exit code (A2).

## Decisions

- **Pipelines are declared apart from steps.** Steps say what runs; pipelines say which steps
  run, in what order and what each waits for. One step serves several execution paths, such as
  a full `ci` and a `quick` run before an agent's turn ends, and a package leaves out a shared
  step by its pipelines alone. A pipeline names the pipelines it depends on, and nothing
  declares what it blocks, so every edge is written where it is used. Rejected: dependencies
  declared on the steps themselves, which tie each step to one order; a plain list run one
  step after another, which serialises independent work; and edges inferred from package
  manifests, as turbo's `^build` does, which hides why a step runs.
- **A pipeline is an ordered list that starts at its first entry, with edges only upward.** A
  pipeline reads top to bottom in an order it can run in, names the step that starts it, and
  cannot hold a cycle, and its written order settles which ready entry starts first, so every
  machine schedules the same way. Rejected: a table of entries keyed by step, whose key order
  TOML does not keep; an ordered list with a separate `start` key and edges pointing anywhere,
  which needs a reachability check and lets the written order disagree with the run order; and
  stages, in which each step waits for the whole stage before it, which loses the edges
  between single steps.
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
- **The scheduler is a state machine in the engine, and every entry runs as a future.** `Run`
  decides what may start and what a failure skips, with no threads and no clock, and its
  rules are tested with fakes. The use case waits on one future for each running entry, a
  tool's or a check step's, so each path through a pipeline advances on its own and a check
  step holds up neither the tools beside it nor a request to stop. The engine needs only the
  language's futures: the runtime that executes them is built outside the cores (B1, B6).
  Rejected: running a check step inline in the use case's loop, which holds up the lines and
  outcomes of the tools running beside it; and threads in the engine.
- **The job limit is the repository's configuration.** `[continual] jobs` sits beside the
  pipelines it bounds, so a terminal, a git hook, an agent's hook and CI share one limit
  without each repeating a flag, and a repository whose steps must not overlap sets it to 1.
  Check steps count like tool steps, since both are work a run waits for. `--jobs` stays for
  one invocation. Rejected: a command-line flag alone, which every hook entry and CI job would
  have to carry; and a limit on tools only, which lets check steps pile up beside them.
- **The engine reaches the checks through a port.** A check step is a name and settings to
  the engine, which asks `CheckRunner` to read and to run them, so the engine changes for no
  check and is tested with a fake. Rejected: check rules or the check registry inside the
  engine, which would make the engine grow with every rule
  ([Workspace](workspace.md#decisions)).
- **Every step runs with a time limit, and the engine reads no clock.** The limit is the
  repository's configuration. A tool's limit goes to the process adapter with its command,
  since measuring time and stopping a process tree is machinery (A3, X1). A check step has
  no process to stop, so the engine races its future against one the `Timer` port hands
  over, and the waiting happens in the execution context. A timed-out step is an ordinary
  failure, so `required` decides what it skips. Rejected: no limit unless configured, which
  leaves a hung step blocking an agent's turn or a CI job until the CI's own timeout; a
  single limit for the whole run, which cannot say which step hung; a way to turn the limit
  off, since a long step sets a long timeout instead; check steps without a limit, bounded
  only by the `git` commands behind them; and reading the time in the engine.
- **Stopping a tool stops its whole process tree.** Test runners and build tools start
  processes of their own, and killing only the parent leaves those running. Rejected: killing
  the direct child only.
- **A step with no matching files is skipped, and that is not a failure.** The entries that
  depend on it still run and the run can pass, so a commit that touches no Rust file is not
  refused by a Rust formatter. It is reported as skipped, never as passed. Rejected: counting
  it as passed, which reports a check that checked nothing; and skipping what depends on it,
  which lets one file filter switch off the rest of a pipeline.
- **Files that do not fit the command line stop the step, unless the step asks for batches.**
  One run of a tool is one judgement, and a tool that reads its files together, such as a
  type checker, judges differently when it gets them in parts, so Continual never splits them
  on its own. `batch = true` is the step saying that its tool judges each file alone.
  Rejected: batching every step, as `xargs` does, which silently changes what such a tool
  checks; and never batching, which leaves a formatter unable to run on a very large commit.

## Open questions

- What a `run` that holds no word is, such as `run = ""`. The engine's configuration input
  holds a tool step's command as a program and its arguments, so a `run` without a program
  cannot reach the engine as a command, and the configuration adapter meets it first, when
  it splits `run` ([Read continual.toml](../tickets/first-release/2-pipeline/read-continual-toml.md)).
- Which task gives the engine a step's `install`. The step definitions hold `run`, `checks`
  and `timeout`, and no ticket of the
  [pipeline story](../tickets/first-release/2-pipeline/story.md) lists `install` among the
  behaviours it adds to the engine.
