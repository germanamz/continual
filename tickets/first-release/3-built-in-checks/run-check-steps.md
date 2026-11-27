---
type: ticket
title: Run check steps
blocks:
  - tickets/first-release/3-built-in-checks/guard-protected-files-in-continual
  - tickets/first-release/3-built-in-checks/check-architecture-in-continual
  - tickets/first-release/3-built-in-checks/split-commit-messages-into-parts
  - tickets/first-release/6-packages/route-files-to-their-packages
  - tickets/first-release/4-events/give-check-steps-an-events-files-and-message
  - tickets/first-release/3-built-in-checks/limit-a-check-steps-time
parent: tickets/first-release/3-built-in-checks/story
points: 3
priority: high
status: pending
---

# Run check steps

## Goal

Let a step run built-in checks without the engine knowing any of them: the engine reads
`checks` entries with their inline settings, asks `CheckRunner` to read them when the
workspace loads and to run them within the pipeline, and prints the finding lines it gets
back. The composition root wires the first check, so `continual run` runs a `comments` check
step.

## Design

[Built-in checks: check steps](../../../docs/built-in-checks.md#check-steps), the `CheckRunner`
row and the `toml-config` note in
[Pipeline: ports and adapters](../../../docs/pipeline.md#ports-and-adapters),
[Pipeline: the engine core](../../../docs/pipeline.md#the-engine-core) and
[composition root](../../../docs/pipeline.md#composition-root).

## Carried over

- A check entry's settings are located values, a type of the library crate that the engine
  holds and hands on without reading
  ([Workspace: library crates](../../../docs/workspace.md#library-crates)). If
  `continual-vocabulary` does not hold it yet, this task declares it there, which is a guarded
  change (H8).

## Behaviours to test

- A check step's entries reach `CheckRunner` in list order, with their package, their
  settings as located values and the run's inputs.
- The finding lines it hands back are printed with paths relative to the repository root, and
  the step fails when its outcome says a check reported findings.
- An outcome that says a check could not run makes the step could-not-run (exit code 2).
- A check step runs as a future beside the tool entries and counts as one job: while its
  checks are still running, the lines and outcomes of the tools beside it keep arriving, and
  a request to stop is honoured.
- An entry that `CheckRunner` rejects, for an unknown check or a setting its check does not
  have, is a configuration error at its line, before anything runs.
- `continual-toml-config` passes a check entry's settings on as located values, and judges
  none of them.
- `--base`, `--range`, `--title`, `--body`, `--ci` and `--labels` are parsed once for every
  command that runs checks, with `origin/main` as the base when none is given, and reach
  `CheckRunner` as the run's inputs.
- The real binary runs a `comments` check step on a fixture repository, and ends with exit
  code 2 on a check step that names no built-in check.

## Done when

- The rules are tested in the engine core with the `CheckRunner` fake, and the engine holds
  no check and no registry code.
- The composition root builds `continual-rust`, `continual-git`, the comments core and its
  bridge, the registry and the `engine-registry` bridge, in the order B5 gives.
