# Workspace

The Cargo workspace that holds Continual: which crates exist, what each one is for, how the
ports-and-adapters architecture maps onto them, and the configuration they share. Read it to
know where code lives. Where a particular change goes is decided by the placement procedure in
[CONVENTIONS.md](../CONVENTIONS.md#5-placement-procedure).

## Concepts

Core, driven port, driving port, adapter and composition root mean what the
[vocabulary](../CONVENTIONS.md#2-vocabulary) says. Domain terms such as check, check name,
finding and pipeline come from the [domain glossary](../CONVENTIONS.md#domain-glossary).

## Behaviour

- `continual` accepts only `--help` and `--version` so far. An unknown flag is a usage error,
  and running it with no arguments prints the help as a usage error (exit code 2).
- The core exposes one value object, `CheckName`, with its error type `CheckNameError`. Its
  doc comments in `crates/continual-core/src/check_name.rs` state the parsing rules.
- Every crate builds on the toolchain pinned in `rust-toolchain.toml`: Rust 1.99.1 with
  rustfmt, clippy and llvm-tools-preview.

## Design

### Crates

| Crate | Path | Role |
| --- | --- | --- |
| `continual-core` | `crates/continual-core/` | The core: domain types and, as they arrive, driven-port traits. No I/O and no technology dependencies. |
| `continual` | `crates/continual/` | The CLI binary. `main.rs` is the composition root and `cli.rs` the driving adapter, built on clap. |
| `xtask` | `xtask/` | Repository tooling, the checks behind `cargo xtask` ([Checks](checks.md)). Product crates never depend on it. |

Adapter crates live under `crates/adapters/<name>/`, and none exist yet. A driven port and its
adapter arrive with the first feature that needs them. The ports are listed in
[Continual's hexagon](../CONVENTIONS.md#3-continuals-hexagon), and the design document of each
area says what its ports ask.

### Planned crates

> Planned: [Run a repository's steps from continual.toml](../tickets/first-release/1-pipeline/story.md)

| Crate | Path | Implements | Designed in |
| --- | --- | --- | --- |
| `continual-toml-config` | `crates/adapters/toml-config/` | `ConfigSource` | [Pipeline](pipeline.md#ports-and-adapters) |
| `continual-process` | `crates/adapters/process/` | `ToolRunner` | [Pipeline](pipeline.md#ports-and-adapters) |
| `continual-terminal` | `crates/adapters/terminal/` | `Report` | [Pipeline](pipeline.md#ports-and-adapters) |
| `continual-git` | `crates/adapters/git/` | `SourceTree`, `ChangeHistory` | [Built-in checks](built-in-checks.md#ports-and-adapters) |
| `continual-rust` | `crates/adapters/rust/` | `LanguageSyntax` for Rust | [Built-in checks](built-in-checks.md#ports-and-adapters) |
| `continual-session-state` | `crates/adapters/session-state/` | `GateState` | [Harness](harness.md#design) |

The core grows the feature modules `workspace`, `pipeline`, `events`, `checks` and `gate`.
`crates/continual` keeps the composition root and two driving adapters: `cli.rs` and
`harness/claude.rs`. Every language adapter is compiled into the one binary
([Distribution](distribution.md#decisions)).

`arch.toml` classifies every crate as core, composition root, adapter or tooling, and lists
the core's allowed external dependencies (none yet) and the couplings allowed between adapters
(none yet). `cargo xtask arch` enforces it ([Checks](checks.md#cargo-xtask-checks)).

### Repository layout

```text
VISION.md                  what Continual does for the people and agents using it
README.md                  what it is, setup, commands, links
CONVENTIONS.md             the rules every contributor follows
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
crates/continual-core/     the core
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
  Launching tools, reading files, git and each language's parser sit behind ports, so one core
  enforces the same rules in every language and a new language is a new adapter. Rejected: a
  core per language, and a lexer or parser inside the core.
- **Crates by role, modules by feature inside the core** (D9, D10). A crate boundary is
  something `cargo xtask arch` can check, while feature modules keep related code together.
- **`CheckName` as the first value object.** The checks needed real logic to exercise:
  tests, coverage and mutation testing. `CheckName` models the name a contributor types to run
  one check, and the first pipeline feature may reshape or replace it. Rejected: proving the
  checks with fixtures only.
- **One binary.** Continual runs from a terminal, a git hook, a CI job or an agent harness, and
  each of those can call the `continual` binary, so there is one composition root. A second
  deployable gets its own when it exists.
- **The repository's own checks stay in `xtask` until the first release.** They are the
  checks Continual generalises. Product crates never depend on `xtask`, so the checks can
  move into Continual without changing the product's architecture, and they move once a
  released `continual` can take over ([Checks: self-hosting](checks.md#self-hosting)).
- **Adapters use the libraries `xtask` already uses.** `toml` and serde for configuration,
  `std::process` for tools and git, and `ra-ap-rustc_lexer`, `syn` and `toml` for Rust, so
  the first adapters add no new dependency.
- **clap with the derive feature**, since every future command needs argument parsing.
- **Smoke tests run the built binary** through `env!("CARGO_BIN_EXE_continual")` and
  `std::process::Command`, so they need no extra test dependency.

## Open questions

- Which shell-word splitting the `toml-config` adapter and the harness adapter use, decided
  with the feature that first needs it.
