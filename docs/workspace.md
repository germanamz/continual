# Workspace

The Cargo workspace that holds Continual: which crates exist, what each one is for, how the
ports-and-adapters architecture maps onto them, and the configuration they share. Read it to
know where code lives. Where a particular change goes is decided by the placement procedure in
[CONVENTIONS.md](../CONVENTIONS.md#5-placement-procedure).

## Concepts

Core, driven port, driving port, adapter, bridge and composition root mean what the
[vocabulary](../CONVENTIONS.md#2-vocabulary) says. Domain terms such as check, check name,
finding and pipeline come from the [domain glossary](../CONVENTIONS.md#domain-glossary).

## Behaviour

- `continual` accepts only `--help` and `--version` so far. An unknown flag is a usage error,
  and running it with no arguments prints the help as a usage error (exit code 2).
- The registry core exposes one value object, `CheckName`, with its error type
  `CheckNameError`. Its doc comments in `crates/cores/registry/src/check_name.rs` state the
  parsing rules.
- Every crate builds on the toolchain pinned in `rust-toolchain.toml`: Rust 1.99.1 with
  rustfmt, clippy and llvm-tools-preview.

## Design

### Crates

| Crate | Path | Role |
| --- | --- | --- |
| `continual-registry` | `crates/cores/registry/` | The registry core, the one core so far: domain types and, as they arrive, driven-port traits. No I/O and no technology dependencies. |
| `continual` | `crates/continual/` | The CLI binary. `main.rs` is the composition root and `cli.rs` the driving adapter, built on clap. |
| `xtask` | `xtask/` | Repository tooling, the checks behind `cargo xtask` ([Checks](checks.md)). Product crates never depend on it. |

Adapter crates live under `crates/adapters/<name>/`, and none exist yet. A driven port and its
adapter arrive with the first feature that needs them. The ports are listed in
[Continual's cores](../CONVENTIONS.md#3-continuals-cores), and the design document of each
area says what its ports ask.

`arch.toml` classifies every crate as core, composition root, adapter or tooling. It names
`crates/cores` as the directory that holds the cores, so every crate under it is a core, and
`crates/adapters` as the one that holds the adapters. It lists each core's allowed external
dependencies (none yet) and the couplings allowed between adapters (none yet).
`cargo xtask arch` enforces it ([Checks](checks.md#cargo-xtask-checks)).

### Planned crates

> Planned: [Run a repository's steps from continual.toml](../tickets/first-release/1-pipeline/story.md)

> Planned: [Run the built-in checks in Continual](../tickets/first-release/2-built-in-checks/story.md)

> Planned: [Hold the review gate in the Claude Code mod](../tickets/first-release/4-claude-code-mod/story.md)

Continual's rules are split into cores. Each is a crate under `crates/cores/<name>/` that
holds one unit of logic and depends on no other workspace crate, another core included.
The registry core is there already and holds `CheckName`. The rest of it and the other cores
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
the other core. A bridge depends on the two cores it joins and on nothing else:

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

### Repository layout

```text
VISION.md                  what Continual does for the people and agents using it
README.md                  what it is, setup, commands, links
CONVENTIONS.md             the rules every contributor follows
MEMORY.md                  what one session hands to the next
AGENTS.md, CLAUDE.md       the agents' short form of CONVENTIONS.md
docs/                      design documents, one per area (this file is one)
tickets/                   planned work, as Tusk tickets
Cargo.toml, Cargo.lock     the workspace, shared lints and dependency versions
rust-toolchain.toml        the pinned toolchain
rustfmt.toml, clippy.toml  formatter settings and lint thresholds
arch.toml                  architecture rules for `cargo xtask arch`
deny.toml                  supply-chain rules for `cargo deny`
committed.toml             commit-message rules for `committed`
lefthook.yml               git hooks
tusk.toml                  the Tusk workspace manifest
.gitattributes             LF line endings on every platform
.cargo/                    the `xtask` alias and cargo-mutants settings
.config/nextest.toml       the nextest CI profile
.claude/                   Claude Code settings, hooks, path-scoped rules and skills
.github/                   CI workflows and the pull-request template
crates/cores/registry/     the registry core, the one core so far
crates/continual/          the CLI binary
xtask/                     the repository checks
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
  hold, and it can be lifted out and used without the others. The price is the number of
  crates, about twenty, built, versioned and published together. Rejected: one core with a
  feature module for each unit, whose boundaries are module visibility that only review
  holds and whose crate grows with every rule; and a core for each of the engine's feature
  modules, since `workspace`, `pipeline` and `events` share one model, the validated graph,
  and a boundary between them would only map that graph back and forth (H5).
- **Cores share no crate.** Each core declares the types its ports speak, and a bridge maps
  one core's types to the other's, so no core changes because another did (H4). A step's name
  in the engine and a check's name in the registry follow the same rules and are separate
  types. Rejected: a shared crate of common types such as findings, names and located values,
  which every core would depend on and none could change alone.
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
- **Cores get libraries through the execution context.** A core that needs what a library
  does, as the commit-messages core needs regular expressions, declares a port for it, and
  the `context` adapter answers. Every core's allow-list stays empty, and the one object the
  composition root builds for a run is where the next such need goes
  ([Built-in checks](built-in-checks.md#decisions)). Rejected: a crate that every core
  depends on and receives when it runs, which cores would share (H4).
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
