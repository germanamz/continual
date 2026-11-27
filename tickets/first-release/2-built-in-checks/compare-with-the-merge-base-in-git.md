---
type: ticket
title: Compare with the merge base in git
blocks:
  - tickets/first-release/2-built-in-checks/judge-with-the-base-guard-settings
  - tickets/first-release/2-built-in-checks/check-commit-messages-in-continual
  - tickets/first-release/2-built-in-checks/join-the-guard-to-the-registry
parent: tickets/first-release/2-built-in-checks/story
points: 2
priority: medium
status: pending
---

# Compare with the merge base in git

## Goal

Implement the guard's `ChangeHistory` in `continual-git`, so the guard
compares a working tree with its merge base against `--base` in a real repository.

## Design

The `git` notes in [Built-in checks: ports and adapters](../../../docs/built-in-checks.md#ports-and-adapters)
and [guard](../../../docs/built-in-checks.md#guard).

## Behaviours to test

- The guard's `ChangeHistory` contract suite passes against the adapter in fixture
  repositories: the merge base, changed and deleted files, and a file's text at a revision
  and in the working tree.
- A base revision missing from a shallow clone is reported as an error naming it.

## Done when

- `continual-git` is created here if it does not exist yet, and `arch.toml` classifies it.
- Existing: the `xtask` guard keeps passing its tests unchanged.
