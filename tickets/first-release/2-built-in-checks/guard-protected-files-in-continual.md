---
type: ticket
title: Guard protected files in Continual
blocks:
  - tickets/first-release/2-built-in-checks/guard-existing-tests-in-continual
  - tickets/first-release/2-built-in-checks/compare-with-the-merge-base-in-git
parent: tickets/first-release/2-built-in-checks/story
points: 3
priority: medium
status: pending
---

# Guard protected files in Continual

## Goal

Start the built-in `guard` check in the core with the part that needs no language: changes to
protected paths and to any `continual.toml`, warning locally and blocking in CI, with the
`ChangeHistory` operations it needs.

## Design

[Built-in checks: guard](../../../docs/built-in-checks.md#guard) and the `ChangeHistory` row in
[ports and adapters](../../../docs/built-in-checks.md#ports-and-adapters).

## Behaviours to test

- A change to a path in `protected`, and to any `continual.toml`, is reported, untracked files
  included.
- A missing base revision makes the check could-not-run, never "no findings".
- Locally the step passes and prints its findings; with `--ci --labels <list>` it fails unless
  the list holds the approval label.
- The fake passes the `ChangeHistory` contract suite for the merge base, changed and deleted
  files, and a file's text at a revision.

## Done when

- The operations are added to `ChangeHistory`, documented per P5; the port is created here if
  it does not exist yet.
