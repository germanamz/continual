# Workspace

The Cargo workspace that holds Continual: which crates exist, what each one is for, how the
ports-and-adapters architecture maps onto them, and the configuration they share. Read it to
know where code lives. Where a particular change goes is decided by the placement procedure in
[CONVENTIONS.md](../CONVENTIONS.md#5-placement-procedure).

## Concepts

Core, driven port, driving port, adapter, bridge, library crate and composition root mean
what the [vocabulary](../CONVENTIONS.md#2-vocabulary) says. Domain terms such as check, check name,
finding and pipeline come from the [domain glossary](../CONVENTIONS.md#domain-glossary).

## Behaviour

- `continual` accepts only `--help` and `--version` so far. An unknown flag is a usage error,
  and running it with no arguments prints the help as a usage error (exit code 2).
- The registry core exposes one value object, `CheckName`, with its error type
  `CheckNameError`. Its doc comments in `crates/cores/registry/src/check_name.rs` state the
  parsing rules.
- The engine core exposes the step definitions of its `workspace` module: `Workspace::build`
  turns configuration input into a workspace of valid steps, or reports every problem with
  its location ([Pipeline](pipeline.md#the-engine-core)).
- The library crate `continual-vocabulary` exposes two plain records: `Location`, a file and
  a line, which prints as `file:line`, and `Located<T>`, a value with the place it is
  written. It also exposes one value object, `Name`, the name rule, with its error type
  `NameError` ([Library crates](#library-crates)).
- Every crate builds on the toolchain pinned in `rust-toolchain.toml`: Rust 1.99.1 with
  rustfmt, clippy and llvm-tools-preview.

## Design

### Crates

| Crate | Path | Role |
| --- | --- | --- |
| `continual-engine` | `crates/cores/engine/` | The engine core: so far the step definitions of its `workspace` module, with the configuration input they are built from and the problems they are rejected with. No I/O and no technology dependencies. |
| `continual-registry` | `crates/cores/registry/` | The registry core: domain types and, as they arrive, driven-port traits. No I/O and no technology dependencies. |
| `continual-vocabulary` | `crates/libraries/vocabulary/` | The library crate: the data several cores mean the same way, so far `Location`, `Located<T>` and `Name`. No I/O and no technology dependencies. |
| `continual` | `crates/continual/` | The CLI binary. `main.rs` is the composition root and `cli.rs` the driving adapter, built on clap. |
| `xtask` | `xtask/` | Repository tooling, the checks behind `cargo xtask` ([Checks](checks.md)). Product crates never depend on it. |

Adapter crates live under `crates/adapters/<name>/`, and none exist yet. A driven port and its
adapter arrive with the first feature that needs them. The ports are listed in
[Continual's cores](../CONVENTIONS.md#3-continuals-cores), and the design document of each
area says what its ports ask.

`arch.toml` classifies every crate as core, library crate, composition root, adapter or
tooling. It names `crates/cores` as the directory that holds the cores, so every crate under
it is a core, `crates/libraries` as the one that holds the library crates and
`crates/adapters` as the one that holds the adapters. It lists the allowed external
dependencies of each core and of each library crate (none yet) and the couplings allowed
between adapters (none yet). `cargo xtask arch` enforces it
([Checks](checks.md#cargo-xtask-checks)).

### Planned crates

> Planned: [Run a repository's steps from continual.toml](../tickets/first-release/2-pipeline/story.md)

> Planned: [Run the built-in checks in Continual](../tickets/first-release/3-built-in-checks/story.md)

> Planned: [Hold the review gate in the Claude Code mod](../tickets/first-release/5-claude-code-mod/story.md)

Continual's rules are split into cores. Each is a crate under `crates/cores/<name>/` that
holds one unit of logic and depends on no other core. Of the workspace's crates it may use
only [library crates](#library-crates).
The registry core is there already and holds `CheckName`, and so is the engine core, which
holds the step definitions of its `workspace` module. The rest of both and the other cores
are planned.

| Core | Path | Holds | Designed in |
| --- | --- | --- | --- |
| `continual-engine` | `crates/cores/engine/` | The feature modules `workspace`, `pipeline` and `events` | [Pipeline](pipeline.md#the-engine-core) |
| `continual-registry` | `crates/cores/registry/` | The check registry and what every check shares | [Built-in checks](built-in-checks.md#the-registry-core) |
| `continual-comments` | `crates/cores/comments/` | The comments check | [Built-in checks](built-in-checks.md#the-check-cores) |
| `continual-architecture` | `crates/cores/architecture/` | The architecture check | [Built-in checks](built-in-checks.md#the-check-cores) |
| `continual-guard` | `crates/cores/guard/` | The guard | [Built-in checks](built-in-checks.md#the-check-cores) |
| `continual-commit-messages` | `crates/cores/commit-messages/` | The commit-messages check | [Built-in checks](built-in-checks.md#the-check-cores) |
| `continual-rust-workspace-lints` | `crates/cores/rust-workspace-lints/` | The `rust-workspace-lints` check | [Built-in checks](built-in-checks.md#the-check-cores) |
| `continual-gate` | `crates/cores/gate/` | The review gate and the blocked commands | [Harness](harness.md#design) |

A core that needs something from outside declares a driven port for it. These adapters answer
the ports that talk to a technology:

| Crate | Path | Implements | Designed in |
| --- | --- | --- | --- |
| `continual-toml-config` | `crates/adapters/toml-config/` | The engine's `ConfigSource` | [Pipeline](pipeline.md#ports-and-adapters) |
| `continual-process` | `crates/adapters/process/` | The engine's `ToolRunner` | [Pipeline](pipeline.md#ports-and-adapters) |
| `continual-terminal` | `crates/adapters/terminal/` | The engine's `Report` | [Pipeline](pipeline.md#ports-and-adapters) |
| `continual-git` | `crates/adapters/git/` | `SourceTree` and `ChangeHistory`, for each core that declares one | [Built-in checks](built-in-checks.md#ports-and-adapters) |
| `continual-rust` | `crates/adapters/rust/` | `LanguageSyntax` for Rust, for each check core that declares one | [Built-in checks](built-in-checks.md#ports-and-adapters) |
| `continual-context` | `crates/adapters/context/` | The execution context: the engine's `Timer` and the commit-messages core's `Patterns` | [Built-in checks](built-in-checks.md#ports-and-adapters) |

A core that needs another core declares a driven port too, and a bridge answers it by calling
the other core. A bridge depends on the two cores it joins and, library crates aside, on
nothing else:

| Bridge | Path | Implements | By calling | Designed in |
| --- | --- | --- | --- | --- |
| `continual-engine-registry` | `crates/adapters/engine-registry/` | The engine's `CheckRunner` and the registry's `BaseEntries` | The registry and the engine | [Built-in checks](built-in-checks.md#bridges) |
| `continual-registry-comments` | `crates/adapters/registry-comments/` | The registry's `Check` | The comments core | [Built-in checks](built-in-checks.md#bridges) |
| `continual-registry-architecture` | `crates/adapters/registry-architecture/` | The registry's `Check` | The architecture core | [Built-in checks](built-in-checks.md#bridges) |
| `continual-registry-guard` | `crates/adapters/registry-guard/` | The registry's `Check` | The guard core | [Built-in checks](built-in-checks.md#bridges) |
| `continual-registry-commit-messages` | `crates/adapters/registry-commit-messages/` | The registry's `Check` | The commit-messages core | [Built-in checks](built-in-checks.md#bridges) |
| `continual-registry-rust-workspace-lints` | `crates/adapters/registry-rust-workspace-lints/` | The registry's `Check` | The `rust-workspace-lints` core | [Built-in checks](built-in-checks.md#bridges) |
| `continual-gate-engine` | `crates/adapters/gate-engine/` | The gate's `GateRules` | The engine | [Harness](harness.md#design) |

One crate under `crates/adapters/` implements no port. `continual-shell-words`, in
`crates/adapters/shell-words/`, splits a command line into words and shell operators, minding
quotes. The `toml-config` adapter splits a step's `run` with it, through a coupling declared
in `arch.toml`, and the harness adapter splits the command an agent wants to run
([Pipeline](pipeline.md#commands), [Harness](harness.md#design)).

`crates/continual` keeps the composition root and two driving adapters: `cli.rs` and
`harness/claude.rs`. `main.rs` is the one place that lists the built-in checks: it builds
each check core, wraps it in its bridge and hands the bridges to the registry. Every check
and every language adapter is compiled into the one binary
([Distribution](distribution.md#decisions)).

Each core arrives with an allow-list of its own in `arch.toml` and a `clippy.toml` like the
registry's ([Checks](checks.md#cargo-xtask-checks)).

### Library crates

A library crate is a third kind of product crate, beside the cores and the adapters. It holds
data that several cores mean the same way, so each of them uses one definition instead of
declaring a copy that a bridge maps. It is no part of the hexagon, neither a port nor an
adapter: to a crate that uses it, it is a dependency, as the standard library is. Library
crates live under `crates/libraries/<name>/`. Cores, adapters and composition roots may all
depend on one, and it depends on no workspace crate.

There is one library crate, `continual-vocabulary` in `crates/libraries/vocabulary/`. It
holds `Location` and `Located<T>`: where something is written, a file and a line, and a value
with the place it is written. Both are plain records, and a `Location` prints as `file:line`.
The engine builds its configuration input and its problems on them, and re-exports both from
its `workspace` module ([open questions](#open-questions)).

It also holds the name rule, `Name`: the lowercase, hyphenated text that names a step, a
check, a pipeline or a package. A name starts with a lowercase ASCII letter and holds only
lowercase ASCII letters, digits and hyphens, each hyphen between two letters or digits.
`Name` is a value object, built with `Name::parse`, and `NameError` is the one error enum
that says why a text is not a name, at its first offending character. Its messages name no
kind of name, so whoever reports one says which kind the text was meant to be: for a step
written as `Fmt`, `NameError` says `name starts with 'F', not a lowercase letter`, and the
engine's problem reads `invalid step name "Fmt": name starts with 'F', not a lowercase
letter`. The engine builds `StepName` on `Name` and re-exports neither `Name` nor
`NameError`, so each has one path ([open questions](#open-questions)).

> Planned: [Share data between cores in library crates](../tickets/first-release/1-library-crates/story.md)

The rest of what the cores share arrives in the same crate:

| It will hold | What it is | Arrives with |
| --- | --- | --- |
| `Finding` | One violation, as a location and a message, which prints as `path:line: message`. A plain record. | The first task that reports one: [Run checks through a registry](../tickets/first-release/3-built-in-checks/run-checks-through-a-registry.md) or [Check comments in Continual](../tickets/first-release/3-built-in-checks/check-comments-in-continual.md) |
| Located values | A check entry's settings and the gate's sections of the configuration, as values that each carry their location, which nothing reads before the check or the gate they belong to. Plain records. | The first task that carries them: [Run check steps](../tickets/first-release/3-built-in-checks/run-check-steps.md), [Run checks through a registry](../tickets/first-release/3-built-in-checks/run-checks-through-a-registry.md) or [Join the gate to the engine](../tickets/first-release/5-claude-code-mod/join-the-gate-to-the-engine.md) |

- **What it may hold** (H6). Plain data records, and value objects whose invariant is about
  the value alone, that two or more cores speak, in the code or in these documents. It
  holds nothing that only one core owns: the engine's `ProblemKind` names the engine's own
  rules, so it stays in the engine, and so does `Problem`, which pairs it with a `Location`.
  It declares no trait and holds no use case, so every port stays in a core, with only the
  operations that core asks (P7).
- **Its limits** (H7). A library crate keeps every limit of a core: no technology
  dependency, no I/O, clock, environment, process or randomness, no `serde`, an allow-list
  of its own in `arch.toml`, a `clippy.toml` of its own and the cores' coverage minimum. It
  depends on no workspace crate, another library crate included.
- **How it changes** (H8). Every change to a library crate is a guarded change, so what the
  cores share changes only with a contributor's approval. A change is also backwards
  compatible: it adds, and never removes, renames or alters what a crate already uses. Cores
  build these records field by field and match their enums without a catch-all arm, so a new
  field on a record or a new variant of an enum is not backwards compatible. Such a change
  arrives as a new item beside the old one, the crates that need it move to it, and the old
  item goes in a planned cleanup, a ticket of its own, once nothing uses it.
- **Each kind of name stays a type of its own.** The engine's `StepName` is built on `Name`,
  so the rule and its error have one definition and a step name still cannot stand in for
  another kind of name.

> Planned: [Build check names on the shared name rule](../tickets/first-release/1-library-crates/build-check-names-on-the-shared-name-rule.md)

- The registry's `CheckName` is built on `Name` as well, in place of the copy of the rule
  and the `CheckNameError` it holds until then.

- **Bridges hand library types on.** A library type crosses a bridge unchanged, as a `String`
  does. A bridge still translates what is each core's own: the inputs and answers of the
  port it implements, error enums, outcomes, a name parsed from text, and settings read from
  located values (H3). The registry's label is its own as well: it pairs a `Finding` with the
  `CheckName` of the check that reported it.

How the checks hold these rules is in [Checks](checks.md#library-crates), and how the crate
is published in [Distribution](distribution.md#design).

### Repository layout

```text
VISION.md                     what Continual does for the people and agents using it
README.md                     what it is, setup, commands, links
CONVENTIONS.md                the rules every contributor follows
MEMORY.md                     what one session hands to the next
AGENTS.md, CLAUDE.md          the agents' short form of CONVENTIONS.md
docs/                         design documents, one per area (this file is one)
tickets/                      planned work, as Tusk tickets
Cargo.toml, Cargo.lock        the workspace, shared lints and dependency versions
rust-toolchain.toml           the pinned toolchain
rustfmt.toml, clippy.toml     formatter settings and lint thresholds
arch.toml                     architecture rules for `cargo xtask arch`
deny.toml                     supply-chain rules for `cargo deny`
committed.toml                commit-message rules for `committed`
lefthook.yml                  git hooks
tusk.toml                     the Tusk workspace manifest
.gitattributes                LF line endings on every platform
.cargo/                       the `xtask` alias and cargo-mutants settings
.config/nextest.toml          the nextest CI profile
.claude/                      Claude Code settings, hooks, path-scoped rules and skills
.github/                      CI workflows and the pull-request template
crates/cores/engine/          the engine core
crates/cores/registry/        the registry core
crates/libraries/vocabulary/  the vocabulary library crate
crates/continual/             the CLI binary
xtask/                        the repository checks
```

The [Workflow](workflow.md) document describes the files under `.claude/`, `.github/`,
`docs/`, `tickets/` and `tusk.toml`. The [Checks](checks.md) document describes the check
configuration.

### Shared configuration

- `[workspace.package]` sets version 0.1.0, edition 2024, `rust-version = "1.99"` and the
  Apache-2.0 license for every crate. The workspace uses resolver 3.
- `[workspace.dependencies]` pins each shared dependency's version once, and every crate sets
  `[lints] workspace = true` so the workspace lints apply everywhere.
- `Cargo.lock` is committed, because the workspace ships binaries.
- `.cargo/config.toml` defines `cargo xtask` as `cargo run --package xtask --`.

## Decisions

- **Rules are domain; tools and languages are technology.** What a check decides, how
  findings are reported, the order a pipeline runs in and when a gate blocks are core types.
  Launching tools, reading files, git and each language's parser sit behind ports, so each
  rule is written once for every language and a new language is a new adapter. Rejected: a
  core per language, and a lexer or parser inside a core.
- **Crates by role, modules by feature inside a core** (D9, D10). A crate boundary is
  something `cargo xtask arch` can check, while feature modules keep related code together.
- **Several small cores, joined by bridges.** Each unit of logic with rules of its own is a
  core: the engine, the check registry, each built-in check and the gate. A core is short
  enough to read whole, its boundary is a crate, which the compiler and `cargo xtask arch`
  hold, and it can be lifted out and used without the other cores, with only the library
  crates it speaks. The price is the number of crates, about twenty, built, versioned and
  published together. Rejected: one core with a
  feature module for each unit, whose boundaries are module visibility that only review
  holds and whose crate grows with every rule; and a core for each of the engine's feature
  modules, since `workspace`, `pipeline` and `events` share one model, the validated graph,
  and a boundary between them would only map that graph back and forth (H5).
- **Cores share library crates, and nothing else.** Data that several cores mean the same
  way has one definition, in a [library crate](#library-crates) that each of them depends on
  (H4, H6). Take the comments check's finding at `src/lib.rs:12`. With a finding type in every
  core, the comments core declares its own, its bridge copies it field by field into the
  registry's, and every other check does the same: six finding types, with two copies of the
  located values and two of the name rule beside them, kept alike by review alone. With a
  library crate there is one `Finding`, and the bridges hand it on. A core still shares no
  rule, no port and no error with another core, and still asks another core through a port
  and a bridge. The price is a crate that most cores depend on and that a core lifted out
  brings along. Rejected: every core declaring its own copies, for the duplication above; and
  stamping each core's copy from one macro, whose crate every core would share anyway and
  whose copies stay separate types that bridges still map.
- **A library crate holds data and the invariants of that data.** That is enough to hold the
  name rule once, and narrow enough to keep every rule a core owns in that core (Q16).
  Rejected: plain data records only, which leaves the name rule written in two cores; port
  traits that several cores declare alike, such as `SourceTree` and `LanguageSyntax`, which
  would give each core the operations of all of them and change a trait every core sees for
  one core's new fact; and anything two cores use, which lets one core's rules in and takes
  the boundaries between cores out.
- **A library crate keeps a core's limits and depends on no workspace crate.** Library crates
  are the foundation every core stands on, so they carry no technology and do not build on
  each other: a dependency between two of them would let a change to one break the other and
  every core above both. A finding holds a location, so the two live in one crate. Rejected:
  library crates that depend on library crates; only cores depending on them, with each core
  re-exporting what its ports speak, which gives a bridge one type under two paths; and limits
  looser than a core's, which would hand a technology to every core at once.
- **Every change to a library crate is guarded and backwards compatible.** Take a `Finding`
  that the registry, five check cores and their bridges build, and a guard task that wants
  findings that only warn. A `severity` field on `Finding` would change every core that builds
  one, though only the guard asked. So a library crate changes only with a contributor's
  approval and only by adding, an old shape goes only in a planned cleanup, and what one
  core's rule needs stays in that core, as a check's answer says that its findings only warn.
  Rejected: holding what a library crate may hold by review alone, which erodes one
  reasonable change at a time; and shapes frozen for good, which leave no way to tidy up.
- **One library crate, split only when a core needs it split.** `continual-vocabulary` holds
  all that the cores share, and splits when a core would depend on it for nothing that core
  uses. Rejected: a narrow crate for each concept, which publishes three or more crates where
  one will do and needs library crates that depend on each other.
- **One name rule, and a type for each kind of name.** A step name, a check name, a pipeline
  name and a package name follow one rule, `Name`, with one error enum whose messages name no
  kind of name. Each core wraps it in types of its own, so a step name is never taken for a
  check name. Rejected: each core keeping an error enum and messages of its own over the
  shared rule, which leaves most of the duplicate in place; one bare `Name` for every kind of
  name; and leaving the rule written in both cores.
- **One bridge crate for each pair of cores.** A bridge can reach only the two cores it joins,
  so a new check adds its core and its bridge and changes no other crate but the composition
  root. Rejected: one crate that holds every check's bridge, which could reach every check
  core at once.
- **`CheckName` as the first value object.** The checks needed real logic to exercise:
  tests, coverage and mutation testing. `CheckName` models the name of one check, so its
  crate is the registry core. Rejected: proving the checks with fixtures only.
- **One binary.** Continual runs from a terminal, a git hook, a CI job or an agent harness, and
  each of those can call the `continual` binary, so there is one composition root. A second
  deployable gets its own when it exists.
- **The repository's own checks stay in `xtask` until the first release.** They are the
  checks Continual generalises. Product crates never depend on `xtask`, so the checks can
  move into Continual without changing the product's architecture, and they move once a
  released `continual` can take over ([Checks: self-hosting](checks.md#self-hosting)).
- **Adapters use the libraries `xtask` already uses.** `toml` and serde for configuration,
  `std::process` for tools and git, and `ra-ap-rustc_lexer`, `syn` and `toml` for Rust. What
  those do not cover is added by the task that first needs it, with its reason (G8): signalling
  a process group, Windows job objects and catching Ctrl-C in the adapters and the composition
  root, where `unsafe` code is forbidden; a runtime that executes a run's futures, in the
  composition root; and regular expressions in the execution context, with globs beside them
  if matching them needs a library.
- **Cores get third-party libraries through the execution context.** A core that needs what
  a third-party library does, as the commit-messages core needs regular expressions, declares
  a port for it, and the `context` adapter answers. Every core's allow-list stays empty, and
  the one object the composition root builds for a run is where the next such need goes
  ([Built-in checks](built-in-checks.md#decisions)). Rejected: a crate that every core
  depends on and receives when it runs, which would hold a port or a technology, and a
  library crate holds neither (H6, H7).
- **One tokenizer of Continual's own splits command lines.** A step's `run` and the command
  an agent wants to run are each one line of text whose quotes must mean the same thing, so
  one small crate reads both: words, quoted text kept together, and shell operators as
  tokens of their own. The configuration adapter rejects an operator, and the harness
  adapter cuts segments at them. The crate sits with the adapters, since shell syntax is a
  format translated at the edge (A1), and `toml-config` reaches it through a declared
  coupling (B4). Rejected: a library such as `shlex` for `run`, which knows no operators, so
  the harness adapter would still need code of its own and an `&&` in a `run` would reach
  the tool as an argument; and the same splitting written in each adapter, which gives
  quotes two readers that can drift apart.
- **clap with the derive feature**, since every future command needs argument parsing.
- **Smoke tests run the built binary** through `env!("CARGO_BIN_EXE_continual")` and
  `std::process::Command`, so they need no extra test dependency.

## Open questions

- Whether a core re-exports the library types its public interface speaks. The engine's
  tests name `Location` and `Located<T>` in its `workspace` module, where the two records
  were declared before they moved, so the engine re-exports them and each has two paths:
  `continual_engine::workspace::Location` and `continual_vocabulary::Location`. One type
  under two paths is the cost for which the decision on a library crate's limits rejects
  cores that re-export. `Name` and `NameError` have one path each: the engine's
  `StepName::parse` and `ProblemKind::InvalidStepName` speak `NameError`, the engine does
  not re-export it, and its tests name `continual_vocabulary::NameError`. Nothing says yet
  whether that is the rule for every library type: which path an adapter or a new test
  writes for the two records, whether their re-export goes once the engine's tests name the
  library crate, which is a guarded change, or whether a core re-exports `Finding` when it
  arrives. The tasks that meet it next:
  [Build check names on the shared name rule](../tickets/first-release/1-library-crates/build-check-names-on-the-shared-name-rule.md)
  and [Read continual.toml](../tickets/first-release/2-pipeline/read-continual-toml.md).
