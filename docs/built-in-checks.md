# Built-in checks

> Planned: [Run the built-in checks in Continual](../tickets/first-release/2-built-in-checks/story.md)

Continual's own checks, for the rules that existing tools do not cover: comments,
architecture, the guard and commit messages ([rules](../VISION.md#rules)). A repository runs
them from check steps in its [pipeline](pipeline.md), with their settings written inline. The
rules are the same in every language; each language is supported by an adapter that reads its
syntax, with Rust first ([languages](../VISION.md#languages)). These checks start as the
`cargo xtask` checks in [Checks](checks.md#cargo-xtask-checks), generalised.

## Concepts

Check, check name, finding, language and guarded change mean what the
[domain glossary](../CONVENTIONS.md#domain-glossary) says. In addition:

- **Check entry**: one item of a check step's `checks` list: the check's name and its
  settings.
- **Check registry**: the built-in checks by name, held by a core of its own that knows none
  of their rules.
- **Per-file check**: a check that inspects files one at a time (comments,
  `rust-workspace-lints`). In an event that carries files it inspects those. Otherwise, in
  `continual run` and in an event without files, it inspects every file of its package that
  git tracks or would track.
- **Repository check**: a check that needs the whole repository or its history
  (architecture, guard, commit messages). It ignores an event's files.
- **Unit**: what the architecture check draws boundaries between: a Rust crate, and in other
  languages the package or module their adapter defines.
- **Layer**: a named group of units with the layers it may depend on.
- **Approval label**: the label a contributor puts on a pull request to approve its guarded
  changes. It is always `guard-approved`, and no setting renames it ([guard](#guard)).

## Behaviour

### Check steps

```toml
[steps.conventions]

[[steps.conventions.checks]]
check = "comments"

[[steps.conventions.checks]]
check = "architecture"
# settings: see Architecture below

[[steps.conventions.checks]]
check = "guard"
protected = ["deny.toml", ".github/**", "xtask/**"]
```

- The checks of a step run in the order the list gives. Every check runs, even after an
  earlier one reports findings, so one run shows them all. The step fails when any check
  reports findings or could not run. Findings that only warn, as the guard's do outside CI,
  are printed and do not fail it ([guard](#guard)).
- Each finding is printed as `path:line: message`, with the path relative to the repository
  root, so editors make it clickable.
- A check step has a `timeout` like a tool step, and one that runs past it fails as timed out
  ([Pipeline: timeouts](pipeline.md#timeouts)).
- A check that cannot run, for example because the base revision is missing in a shallow
  clone, says why, and its step could not run (exit code 2). It never reports "no findings"
  instead.
- No check passes vacuously. A per-file check with no file in a supported language to inspect
  fails, and so does an architecture check whose configured units do not exist. In an event
  that carries files, a per-file check with none of them in a supported language is skipped,
  not passed, while the repository checks of its step still run. A step is skipped only when
  every check in it was, and a skipped step does not fail the run
  ([Pipeline: placeholders](pipeline.md#placeholders)).
- Paths in a check's settings are relative to the package that holds the step.
- A `check` that names no built-in check, and a setting its check does not have, are
  configuration errors, reported with their lines before anything runs.
- A file whose language has no adapter is not inspected by per-file checks.

### Comments

No comment blocks, no block comments and no commented-out code; explanations go in doc
comments. The check reports a blob of two or more consecutive lines that hold only a non-doc
line comment once, with its length; any block comment, a nested one counting once; and a line
comment that reads as code. Doc comments are never reported, comment markers inside strings are
never comments, and CRLF files are checked like LF files. Which comments read as code is the
language adapter's to say, since each language writes code differently: the Rust adapter marks
a comment that ends in `;`, `{` or `}`, as `xtask` does today.

### Language-specific checks

A rule that only makes sense in one language is a check of its own, named for its language,
and a repository runs it in a step of its own, so the graph keeps it apart from the checks
every language shares. A repository without that language never lists it.

- **`rust-workspace-lints`**: every crate inherits the workspace lints with
  `[lints] workspace = true`, as the `xtask` arch check requires today. It is a per-file
  check that inspects each crate's manifest.

Its rule is a core of its own like every other check's, over facts that only that language's
adapter reports.

### Architecture

The dependency boundaries a repository declares, such as a core that depends on no technology
and adapters that do not depend on each other. The settings name layers, the units in each and
what each may depend on:

```toml
[[steps.conventions.checks]]
check = "architecture"

[[steps.conventions.checks.layers]]
name = "cores"
units = ["crates/cores/*"]
independent = true
external = []
forbidden_imports = ["std::fs", "std::net", "std::env", "std::process", "std::io", "std::thread"]

[[steps.conventions.checks.layers]]
name = "adapters"
units = ["crates/adapters/*"]
may_depend_on = ["cores"]
independent = true

[[steps.conventions.checks.layers]]
name = "composition-roots"
units = ["continual"]
may_depend_on = ["cores", "adapters"]

[[steps.conventions.checks.layers]]
name = "tooling"
units = ["xtask"]
```

- A layer names its units by name, such as `continual`, or by a glob of the directories that
  hold them, such as `crates/adapters/*`.
- Every unit must belong to a layer, and every unit a layer names must exist.
- A unit depends only on its own layer's units and on the layers in `may_depend_on`. In an
  `independent` layer, units do not depend on each other except through declared
  `couplings`, each with a reason.
- `external`, when present, is the allow-list of external dependencies, each with a reason.
  Without it, a layer's external dependencies are not checked.
- `forbidden_imports` lists import paths that the layer's source files must not use.
- A unit may depend on itself, to enable test features.

This repository's `arch.toml` becomes this check's settings in `continual.toml`.

### Guard

Changes to existing tests and to the checks' own configuration need a contributor's approval.
The guard compares the working tree, untracked files included, with its merge base against
`--base` (`origin/main` by default) and reports:

- an existing test deleted, renamed, ignored, made to expect a panic or failure, or changed in
  any token (reformatting is not a change), and a deleted file that held tests;
- in test files and test modules, changed helpers and file or module attributes;
- a new lint suppression, and a change that loosens the language's lint configuration;
- any change to a path in `protected`, and to any `continual.toml`, which is always protected.

Adding a test is never reported. A source file it cannot parse is reported, not skipped.
Locally the guard only warns: its findings are printed and its step passes, so an approved
change never leaves an agent stuck. With `--ci --labels <list>`, the step fails unless the list
holds the approval label, `guard-approved`.

The guard reads its own settings from the base revision's `continual.toml`, not the working
tree's, so a change cannot loosen the guard that judges it. When the base revision has no
`continual.toml`, or none with a guard entry, the guard judges with its built-in settings: no
`protected` path beyond `continual.toml` itself. The change that gives a repository its first
guard entry is therefore a guarded change like any other, reported for its `continual.toml`.

The approval label is not one of those settings. The guard honours one name,
`guard-approved`, in every repository, and an agent's command may never name it
([Harness: blocked commands](harness.md#blocked-commands)). A label that a repository could
rename would open a gap. Take a repository whose `main` calls the label `guard-approved`, and
a change that renames it to `reviewed`. In CI the guard judges that change with the base
revision's settings, so it still honours `guard-approved`. In the agent's session the gate
reads the working tree, so it would block `reviewed` and stop blocking `guard-approved`. The
agent could then put `guard-approved` on its own pull request and approve the change that
weakened the guard. A change that removes the guard entry would do the same. With one fixed
name, the guard and the gate cannot disagree about it.

`continual guard --ci --labels <list>` runs the guard on its own, outside any step, with the
same settings, so CI runs it even when a change removes it from the steps. With several guard
entries at the base revision, such as one in each package, it runs each with its own
settings.

### Commit messages

Commits and pull requests follow the repository's message rules. The check reads the message
file of the `on_commit_msg` event, every commit in `--range`, and the pull request's
`--title` and `--body`. A pull request's title counts as a subject and its description as a
body.

```toml
[[steps.commit-messages.checks]]
check = "commit-messages"
attribution = true

[[steps.commit-messages.checks.forbidden]]
part = "subject"
pattern = '(?i)^wip\b'
reason = "finish the work before committing it"

[[steps.commit-messages.checks.required]]
part = "body"
pattern = 'tickets/[a-z0-9-]+'
reason = "name the ticket the change works on"
```

- **Attribution** is on unless `attribution = false`. It rejects AI co-author trailers,
  session trailers and links, and "generated with" lines; human co-authors and ordinary prose
  pass.
- **`forbidden`** patterns must not match their part, and each match is a finding.
- **`required`** patterns must match their part at least once, and a message where one does
  not is a finding.
- `part` is `subject` (the first line), `body` (the lines after the first blank line, trailers
  excluded), `trailers` or `message` (all of it). Every finding carries the pattern's
  `reason`.
- A pattern that is not a valid regular expression is a configuration error, reported with its
  line before anything runs.

Conventional Commits stay with the tools that already check them, such as `committed`. On
`on_commit_msg` they run as tool steps that take `{message_file}`. Over a pull request's
commits they stay a CI job of their own until tool steps have a placeholder for the range
([Pipeline: placeholders](pipeline.md#placeholders)).

## Design

The engine runs a check step by asking the check registry, a core of its own, and the
registry asks each check, which is a core of its own too. No core depends on another: each
pair meets in a bridge ([Workspace](workspace.md#planned-crates)).

### The registry core

`continual-registry` is the check registry. It holds the built-in checks by name and what
every check shares, and none of their rules. It reaches a check through one driven port,
`Check`, which says in the registry's words what any check is:

- its `CheckName`, and whether it is a per-file check or a repository check;
- whether it judges with the working tree's settings or with the base revision's, as the
  guard does;
- its reading of a check entry's settings. They reach the check as located values, which
  nothing before it interprets, and it accepts them or rejects an unknown key or a wrong
  value with its line;
- its inspection. A per-file check takes one file's path and text and answers with findings,
  or says that no language adapter reads the file. A repository check takes its package's
  directory and the run's inputs, such as the base revision and the commit range
  ([Pipeline](pipeline.md#the-engine-core)), and answers with findings. Either says when its
  findings only warn and do not fail the step, as the guard's do outside CI, and either can
  answer that it could not run, and why.

The registry is built from the `Check` implementations the composition root hands it, and
refuses two with the same name. Around them it does what every check shares. It finds each
entry's check by name and has the check read the entry's settings. It chooses the files a
per-file check inspects, reads them through `SourceTree` and applies the skip and
no-vacuous-pass rules to them. It runs a step's checks in list order and labels each
`Finding` with its check's `CheckName`. For a check that judges with the base revision's
settings, it asks `BaseEntries` for the entries that check has in the step's package at the
base revision, and runs the check once with each, or once with the check's built-in settings
when the package has none there. `continual guard` asks the registry to run one check by name
the same way, outside any step: once for each entry the check has at the base revision, in
every package.

The registry names no check. Its tests hand it fake checks, and the composition root hands
it the built-in ones.

### The check cores

Each built-in check is a core of its own: `continual-comments`, `continual-architecture`,
`continual-guard`, `continual-commit-messages` and `continual-rust-workspace-lints`. A check
core holds:

- its settings, as typed values with their defaults;
- its rule, a function from facts to findings, written once for every language;
- its loading, a use case that asks the core's own ports for the facts the rule needs and
  passes them to it. The rule never calls a port itself (M5).

A check core knows nothing of the registry, of steps or of how `continual.toml` is written,
so it can be used without the rest of Continual. A check that reads source text holds one
`LanguageSyntax` implementation for each language and chooses among them by a file's
extension.

Adding a check is a new core, its bridge and one line in the composition root. No other core
or adapter changes, unless the rule needs a fact that no adapter reads yet.

### Ports and adapters

| Core | Port | What the core asks | Adapter |
| --- | --- | --- | --- |
| registry | `Check` | One built-in check: its name and kind, where its settings come from, its reading of an entry's settings and its findings | `registry-<check>`, the check's bridge |
| registry | `BaseEntries` | The entries one check has in the configuration at the merge base with a revision, each with its package and its settings as located values | `engine-registry`, a bridge |
| registry | `SourceTree` | The files git tracks or would track under a directory, and a file's text | `crates/adapters/git` |
| comments | `LanguageSyntax` | For one language, from a file's text: its comments, and which of them read as code | `crates/adapters/rust` |
| architecture | `SourceTree` | The files git tracks or would track under a directory, and a file's text | `crates/adapters/git` |
| architecture | `LanguageSyntax` | For one language: from a manifest's text, with the text of the workspace manifest it inherits from, the unit it declares and that unit's dependencies; from a file's text, its imports | `crates/adapters/rust` |
| guard | `ChangeHistory` | The merge base with a revision, the files changed or deleted since it, and a file's text at that revision and in the working tree | `crates/adapters/git` |
| guard | `LanguageSyntax` | For one language, from a file's text: its tests and its lint suppressions; and whether a change loosens the language's lint configuration | `crates/adapters/rust` |
| commit-messages | `ChangeHistory` | The messages of the commits in a range, and the message in a commit message file | `crates/adapters/git` |
| commit-messages | `Patterns` | Whether a pattern's text is a valid pattern, and why not when it is not; and whether a valid pattern matches a text | `crates/adapters/context` |
| rust-workspace-lints | `LanguageSyntax` | From a Rust manifest's text: whether its crate inherits the workspace lints | `crates/adapters/rust` |

- A port that several cores declare is a trait in each of them, with only the operations
  that core asks, and its contract suite and fake live in that core (T6). One adapter crate
  implements them all, since they talk to one actor.
- The `git` adapter runs the `git` command-line tool, as `xtask` does today. One crate
  implements every `SourceTree` and `ChangeHistory`. It runs git with `GIT_TERMINAL_PROMPT=0`
  and a fixed time limit of its own (A3), and reports a git command that times out as an
  error, so a credential prompt or a stuck repository makes a check could-not-run instead of
  hanging.
- `LanguageSyntax` is pure: text in, facts out, with no file system and no processes. Like
  tree-sitter's queries, it speaks in generic concepts (comment, comment that reads as code,
  import, test, lint suppression, unit, dependency) and each language adapter fills them in from
  its own syntax. Each adapter states the file extensions and manifest names it handles, and a
  check core chooses the adapter by extension. The composition root hands each check core
  every language adapter compiled into the binary.
- The `rust` adapter lexes comments with `ra-ap-rustc_lexer`, reads imports, tests and
  suppressions with `syn`, and reads crates and their dependencies from `Cargo.toml`
  manifests with `toml`, the libraries `xtask` uses today.
- The `context` adapter is the execution context: the one object the composition root builds
  for what a core needs from the machinery of a run and not from the repository. It answers
  the engine's `Timer`, which ends a check step that runs past its time limit
  ([Pipeline](pipeline.md#ports-and-adapters)), and the commit-messages core's `Patterns`,
  with the `regex` crate. A core that needs a library asks for it this way, through a port
  of its own, so no core carries an external dependency.
- A step's checks are one future of the run ([Pipeline](pipeline.md#the-engine-core)). The
  operations that wait on the repository are asynchronous: `SourceTree`, `ChangeHistory`,
  `BaseEntries` and a check's inspection through `Check`. A check that waits for git
  therefore lets the rest of the run go on. `LanguageSyntax` and `Patterns` only compute,
  and answer at once.
- Test items carry a token fingerprint, so the guard can tell a changed test from a
  reformatted one without knowing the language.
- The guard's base settings come through the registry. The guard says that it judges with
  the base revision's settings, and the registry asks `BaseEntries` for the guard's entries.
  The `engine-registry` bridge answers by asking the engine, which reads the base revision's
  `continual.toml` files through its own `ChangeHistory` and `ConfigSource`
  ([Pipeline](pipeline.md#the-engine-core)). The guard then reads those settings as it reads
  the working tree's.

### Bridges

A bridge joins two cores. It is a driven adapter of the core whose port it implements and a
driving adapter of the core it calls, and it only translates between their types (H2, H3).

- **`engine-registry`** holds one adapter for each direction. One implements the engine's
  `CheckRunner` ([Pipeline](pipeline.md#ports-and-adapters)) by calling the registry: it
  turns a check step's entries, package, files and inputs into the registry's input, and
  hands each finding back as a line of the step's output, `path:line: message`, with the
  step's outcome. The other implements the registry's `BaseEntries` by calling the engine's
  reader of the base revision's entries, which needs no registry, so each adapter is built
  after the core it calls and before the core it serves (B5).
- **`registry-<check>`**, one crate for each check, implements the registry's `Check` by
  calling its check core. It states the check's name and kind, and reads an entry's located
  values into the check's settings: it rejects an unknown key or a wrong type with its line
  (V1), and gives the line of any setting the check core itself rejects. It maps the check's
  findings to the registry's.

`main.rs` builds each check core with the adapters it needs, wraps it in its bridge and
hands the bridges to the registry, so the composition root is the one place that lists the
built-in checks.

## Decisions

- **Built-in checks are steps.** They sit in pipelines like tools, with dependencies and
  `required`, and a step can run several of them. Rejected: a separate section for Continual's
  own checks, which would give them a second scheduling model.
- **Generic concepts, filled in by each language's adapter.** A check core knows comments,
  code, imports, tests and units; what each looks like in a language is the adapter's to say,
  as a tree-sitter grammar fills in a query. Rejected: recognising code in comments with one
  pattern for every language, which only fits the languages that end statements in `;`.
- **A language-only rule is its own check, in its own step.** It runs only where the language
  is, and the pipeline keeps it apart from the shared checks. Rejected: language-specific
  options on a shared check, which give a shared rule settings that most languages ignore.
- **"Check", not "rule".** A rule is a convention with an ID such as B1, which a check
  enforces, so the configuration key is `check`. Rejected: `rule = "comments"`, which gives
  one word two meanings.
- **The rules live in cores; languages supply facts.** One implementation of each rule
  serves every language, and a new language is a new adapter. Rejected: one implementation
  per language, which would let the rules drift apart.
- **The registry is a core of its own, and so is each check.** The registry holds what
  checks share and reaches each one through `Check`, a driven port with one implementation
  for each built-in check. A check keeps its settings, its loading and its rule in one small
  core that depends on nothing else, so it reads whole, cannot reach into another check and
  can be used without Continual. Adding one touches its own two crates and one line of the
  composition root. The price is two crates for every rule, built and published with the
  rest. Rejected: a use case that calls each check by name, beside a configuration adapter
  that knows each check's settings, which spreads every check over the adapter, the
  configuration types, the use case and its own module; checks as modules behind a trait
  inside one core, whose boundaries are module visibility that only review holds and whose
  crate grows with every rule; and checks as plain adapters, which is where technology lives,
  so a rule could read files itself and each language could grow a copy of its own.
- **Each core declares the ports it needs, with only the operations it asks.**
  `LanguageSyntax` is a small trait in each check core that reads source, and the `rust`
  adapter implements them all. A check core then depends on nothing it does not use, and a
  new fact for one check changes one core. Rejected: one wide port shared by every check,
  which needs a crate that every check core depends on (H4).
- **A bridge reads a check's settings from located values.** The shape of a check's settings
  is part of how `continual.toml` is written, so the bridge rejects an unknown key or a
  wrong type (V1), and the check core rejects a value that breaks its rule, such as a layer
  that names no unit (V3). Rejected: located values inside the check core, which would tie
  it to the configuration file.
- **Manifests are read as text.** The Rust adapter reads `Cargo.toml` files instead of running
  `cargo metadata`, so `LanguageSyntax` stays pure and needs no tool. Rejected: `cargo
  metadata`, which would make a language adapter launch processes.
- **The guard judges with the base revision's settings, every `continual.toml` is protected,
  and CI runs the guard outside the steps.** A change that removes a protected path, a check
  or the guard itself is a guarded change that the unchanged guard reports. Rejected: reading
  the guard's settings from the working tree, which lets a change approve itself; running
  the guard in CI only as a step, which a change could remove; and refusing to run when the
  base revision has no guard entry, which the change that introduces the guard could never
  pass.
- **The approval label is one fixed name.** `guard-approved` is not a setting, because a
  setting can be read from two places that disagree. The guard must judge with the base
  revision's settings, so that a change cannot loosen the guard that judges it, while the
  gate answers in an agent's session, from the working tree. A change that renamed the label,
  or removed the guard entry, would leave the gate blocking the new name while the guard
  still honoured the old one, and the agent could approve its own change
  ([guard](#guard)). Rejected: a configurable label that the gate blocks from the working
  tree's settings, which opens that gap; and having the gate read the label from another
  branch or revision, such as the base revision the guard uses. That narrows the gap without
  closing it. The gate would read its own clone's copy of that branch, which can be behind
  the remote or missing, while the guard in CI judges with the pull request's real merge
  base, and a pull request can target a branch other than the one the gate read. It would
  also put a `git` command before every command an agent runs.
- **Conventional Commits stay with existing tools.** Continual adds the attribution rule that
  tools do not check, and a repository's own forbidden and required patterns, and runs
  `committed` or commitlint as tool steps for the rest. Rejected: reimplementing a
  commit-message linter.
- **Message patterns are regular expressions, matched through the execution context.** Which
  part of a message a pattern applies to, and what a match or a miss means, is the
  commit-messages core's rule. Reading and matching the expression is asked of `Patterns`, a
  port of that core, which the `context` adapter answers with the `regex` crate, a third
  party to the core (P4). Every core then keeps an empty allow-list, so the architecture
  check needs no allow-list for one unit of a layer, and the next thing a core needs from a
  library has a place to go. Rejected: `regex` on the commit-messages core's allow-list in
  `arch.toml`, which makes one core the exception among cores that allow nothing; and
  compiling patterns in the configuration adapter, which would tie the check to
  `continual.toml`.
- **Attribution is on by default.** A repository that wants AI attribution in its history says
  so with `attribution = false`.
