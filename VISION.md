# Continual

Continual is an xtask orchestrator for any language, written in Rust. Agents follow a
repository's rules consistently when a tool enforces them, not when a document describes them.
Continual does not replace the formatters, linters and test runners a repository already
uses: it runs them, adds the checks they do not cover, and enforces the results the same way
every time, on every machine and in CI.

## Behaviours

### Pipeline

A repository declares its steps in one `continual.toml`, each by name, like the scripts of a
`package.json`, and declares its pipelines apart from them: which steps each pipeline runs and
what each step waits for, so the same steps can run along different paths, such as a full CI
run and a quick check before an agent's turn ends. Continual runs each pipeline as a graph:
steps that do not wait for each other run at the same time, and a failing step that is
required skips every step downstream of it. A pipeline can depend on other pipelines. In a
monorepo, each package's `continual.toml` extends the root one, and a pipeline can depend on
another package's pipeline. The pipelines that run locally are the ones CI runs, so a change
that passes on a contributor's machine passes in CI. When a tool a step needs is missing,
Continual prints the command that installs it.

Continual is an orchestrator, not a build system: it never caches, and every step runs each
time it is asked for, leaving incremental work to the tools. It passes each tool's output
through as the tool writes it and reports every failure by its exit code.

### Checks

A check inspects the repository and either passes or reports findings, each one as
`path:line: message` so editors make it clickable. A check with findings fails; a check that
cannot run says why and never reports "no findings" instead. No check passes vacuously: a
check fails when what it checks is missing.

### Tools

Continual runs the repository's existing tools as steps, passes their output through untouched
and judges each one by its exit code. Which tools run, with which arguments, is the
repository's configuration, and so are the thresholds the tools enforce, such as a minimum
coverage or no missed mutants in mutation testing, set through the tools' own flags.

### Rules

Continual has its own checks for rules that existing tools do not cover:

- **Comments**: no comment blocks, no block comments and no commented-out code;
  explanations go in doc comments.
- **Architecture**: the dependency boundaries a repository declares, such as a core that
  depends on no technology and adapters that do not depend on each other.
- **Guard**: changes to existing tests and to the checks' own configuration need a
  contributor's approval. The guard warns locally and blocks in CI until a contributor
  approves the change.
- **Commit messages**: commits and pull requests follow the repository's message rules,
  such as Conventional Commits and no AI attribution, configurable by the user.

### Languages

The rules are the same in every language. Each language is supported by an adapter that
knows its syntax and its tools; Rust is the first.

### Hooks

The same checks run from git hooks, from CI and from agent harnesses, with one
implementation behind all of them. A repository maps moments such as a commit, an agent's
file edit or the end of an agent's turn to the steps they run, and a step can receive the
files the moment touched.

## Interfaces

### Humans

- The `continual` command-line interface: a single executable, installed with cargo, built
  from source or downloaded, which a repository pins to one release.

### AI

#### Harness Mods/Plugins

Harness mods, starting with Claude Code, that format and check each file an agent edits,
run the quick checks before an agent's turn ends, block commands that skip checks, and hold
the review gate: an agent commits, pushes or opens a pull request only when a contributor
asks, unless explicitly allowed by the user.

### CI

CI workflows run the same pipeline as a contributor's machine, plus the gates that need a
contributor's approval.

## Extending this document

Contributors add to this file as Continual gains behaviours. Describe what Continual does for
the person or agent using it. How it is built belongs in [CONVENTIONS.md](CONVENTIONS.md)
and in the design documents under `docs/`.
