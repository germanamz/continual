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
- **Per-file check**: a check that inspects files one at a time (comments). In an event it
  inspects the event's files; otherwise every file of its package that git tracks or would
  track.
- **Repository check**: a check that needs the whole repository or its history
  (architecture, guard, commit messages). It ignores an event's files.
- **Unit**: what the architecture check draws boundaries between: a Rust crate, and in other
  languages the package or module their adapter defines.
- **Layer**: a named group of units with the layers it may depend on.
- **Approval label**: the label a contributor puts on a pull request to approve its guarded
  changes, `guard-approved` by default.

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
approval-label = "guard-approved"
protected = ["deny.toml", ".github/**", "xtask/**"]
```

- The checks of a step run in the order the list gives. Every check runs, even after an
  earlier one reports findings, so one run shows them all. The step fails when any check
  reports findings or could not run.
- Each finding is printed as `path:line: message`, with the path relative to the repository
  root, so editors make it clickable.
- A check that cannot run, for example because the base revision is missing in a shallow
  clone, says why, and its step could not run (exit code 2). It never reports "no findings"
  instead.
- No check passes vacuously. A per-file check with no file in a supported language to inspect
  fails, and so does an architecture check whose configured units do not exist. In an event,
  a check step with no matching files is skipped, not passed.
- Paths in a check's settings are relative to the package that holds the step.
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
  `[lints] workspace = true`, as the `xtask` arch check requires today.

Its rule is core code like every other check's, over facts that only that language's adapter
reports.

### Architecture

The dependency boundaries a repository declares, such as a core that depends on no technology
and adapters that do not depend on each other. The settings name layers, the units in each and
what each may depend on:

```toml
[[steps.conventions.checks]]
check = "architecture"

[[steps.conventions.checks.layers]]
name = "core"
units = ["continual-core"]
external = []
forbidden-imports = ["std::fs", "std::net", "std::env", "std::process", "std::io", "std::thread"]

[[steps.conventions.checks.layers]]
name = "adapters"
units = ["crates/adapters/*"]
may-depend-on = ["core"]
independent = true

[[steps.conventions.checks.layers]]
name = "composition-roots"
units = ["continual"]
may-depend-on = ["core", "adapters"]

[[steps.conventions.checks.layers]]
name = "tooling"
units = ["xtask"]
```

- Every unit must belong to a layer, and every unit a layer names must exist.
- A unit depends only on its own layer's units and on the layers in `may-depend-on`. In an
  `independent` layer, units do not depend on each other except through declared
  `couplings`, each with a reason.
- `external`, when present, is the allow-list of external dependencies, each with a reason.
  Without it, a layer's external dependencies are not checked.
- `forbidden-imports` lists import paths that the layer's source files must not use.
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
holds the approval label.

The guard reads its own settings from the base revision's `continual.toml`, not the working
tree's, so a change cannot loosen the guard that judges it. `continual guard --ci --labels
<list>` runs the guard on its own, outside any step, with those settings, so CI runs it even
when a change removes it from the steps. It fails when the base revision's configuration has
no guard entry.

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

Conventional Commits stay with the tools that already check them, such as `committed`, run as
tool steps.

## Design

### Core

The rules are core code, in a `checks` feature module with one submodule per check, written
once for every language. Each one is a function from data to findings: the core use case loads
the files, their base versions and the language facts through the ports below and passes them
in (M5). Findings are the core's `Finding` values, labelled with the check's `CheckName`.

### Ports and adapters

| Port | What the core asks | Adapter |
| --- | --- | --- |
| `SourceTree` | The files git tracks or would track under a directory, and a file's text | `crates/adapters/git` |
| `ChangeHistory` | The merge base with a revision, a file's text at a revision, the files changed or deleted since it, and the messages of the commits in a range | `crates/adapters/git` |
| `LanguageSyntax` | For one language, from a file's text: its comments, imports, tests and lint suppressions; from a manifest's text: the unit it declares and that unit's dependencies | `crates/adapters/rust` |

- The `git` adapter runs the `git` command-line tool, as `xtask` does today. One crate
  implements both ports, since both talk to the same actor. It runs git with
  `GIT_TERMINAL_PROMPT=0` and a fixed time limit of its own (A3), and reports a git command that
  times out as an error, so a credential prompt or a stuck repository makes a check
  could-not-run instead of hanging.
- `LanguageSyntax` is pure: text in, facts out, with no file system and no processes. Like
  tree-sitter's queries, it speaks in generic concepts (comment, comment that reads as code,
  import, test, lint suppression, unit, dependency) and each language adapter fills them in from
  its own syntax. Each adapter states the file extensions and manifest names it handles, and
  the core chooses the adapter by extension. The composition root registers every language
  adapter compiled into the binary.
- The `rust` adapter lexes comments with `ra-ap-rustc_lexer`, reads imports, tests and
  suppressions with `syn`, and reads crates and their dependencies from `Cargo.toml`
  manifests with `toml`, the libraries `xtask` uses today.
- Test items carry a token fingerprint, so the core can tell a changed test from a reformatted
  one without knowing the language.

## Decisions

- **Built-in checks are steps.** They sit in pipelines like tools, with dependencies and
  `required`, and a step can run several of them. Rejected: a separate section for Continual's
  own checks, which would give them a second scheduling model.
- **Generic concepts, filled in by each language's adapter.** The core knows comments, code,
  imports, tests and units; what each looks like in a language is the adapter's to say, as a
  tree-sitter grammar fills in a query. Rejected: recognising code in comments with one
  pattern for every language, which only fits the languages that end statements in `;`.
- **A language-only rule is its own check, in its own step.** It runs only where the language
  is, and the pipeline keeps it apart from the shared checks. Rejected: language-specific
  options on a shared check, which give a shared rule settings that most languages ignore.
- **"Check", not "rule".** A rule is a convention with an ID such as B1, which a check
  enforces, so the configuration key is `check`. Rejected: `rule = "comments"`, which gives
  one word two meanings.
- **The rules live in the core; languages supply facts.** One implementation of each rule
  serves every language, and a new language is a new adapter. Rejected: one implementation
  per language, which would let the rules drift apart.
- **Manifests are read as text.** The Rust adapter reads `Cargo.toml` files instead of running
  `cargo metadata`, so `LanguageSyntax` stays pure and needs no tool. Rejected: `cargo
  metadata`, which would make a language adapter launch processes.
- **The guard judges with the base revision's settings, every `continual.toml` is protected,
  and CI runs the guard outside the steps.** A change that removes a protected path, a check
  or the guard itself is a guarded change that the unchanged guard reports. Rejected: reading
  the guard's settings from the working tree, which lets a change approve itself; and running
  the guard in CI only as a step, which a change could remove.
- **Conventional Commits stay with existing tools.** Continual adds the attribution rule that
  tools do not check, and a repository's own forbidden and required patterns, and runs
  `committed` or commitlint as tool steps for the rest. Rejected: reimplementing a
  commit-message linter.
- **Message patterns are regular expressions, matched in the core.** Matching a message
  against a pattern is a rule, and the `regex` crate is pure computation, so it goes on the
  core's allow-list in `arch.toml` with that reason, with the contributor's approval, in the
  change that builds the check. Rejected: compiling patterns in the configuration adapter
  behind a port, which adds a port with no external actor (P4).
- **Attribution is on by default.** A repository that wants AI attribution in its history says
  so with `attribution = false`.
