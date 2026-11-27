---
type: ticket
title: Inspect every tracked file with a per-file check
blocks:
  - tickets/first-release/3-built-in-checks/run-check-steps
  - tickets/first-release/3-built-in-checks/join-the-comments-check-to-the-registry
parent: tickets/first-release/3-built-in-checks/story
points: 3
priority: high
status: pending
---

# Inspect every tracked file with a per-file check

## Goal

Add the registry's `SourceTree` port and its `continual-git` implementation, and the rules
every per-file check shares: the registry hands the check each file git tracks or would
track, and no per-file check passes without a file to inspect.

## Design

The per-file check in [Built-in checks: concepts](../../../docs/built-in-checks.md#concepts),
[check steps](../../../docs/built-in-checks.md#check-steps), and the registry's `SourceTree`
row and the `git` notes in
[ports and adapters](../../../docs/built-in-checks.md#ports-and-adapters).

## Behaviours to test

- Every tracked file and every untracked file that is not ignored is listed; ignored files are
  not.
- A per-file check is handed each file's path and text, and a file the check answers as not
  read by any language adapter counts as not inspected.
- With no file inspected, the check fails instead of passing.
- A file that disappears between listing and reading is reported, not skipped silently.
- The `SourceTree` contract suite passes against the fake and the `continual-git` adapter.

## Done when

- The rules are tested in the registry core with a fake check.
- `continual-git` is created here if it does not exist yet, and `arch.toml` classifies it.
