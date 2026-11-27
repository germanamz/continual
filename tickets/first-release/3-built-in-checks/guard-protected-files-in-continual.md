---
type: ticket
title: Guard protected files in Continual
blocks:
  - tickets/first-release/3-built-in-checks/guard-existing-tests-in-continual
  - tickets/first-release/3-built-in-checks/compare-with-the-merge-base-in-git
  - tickets/first-release/5-claude-code-mod/block-commands-that-skip-checks
  - tickets/first-release/3-built-in-checks/join-the-guard-to-the-registry
parent: tickets/first-release/3-built-in-checks/story
points: 3
priority: medium
status: pending
---

# Guard protected files in Continual

## Goal

Create the guard core, `continual-guard`, with the part that needs no language: changes to
protected paths and to any `continual.toml`, warning locally and blocking in CI, with the
`ChangeHistory` port it needs.

## Design

[Built-in checks: guard](../../../docs/built-in-checks.md#guard), [the check cores](../../../docs/built-in-checks.md#the-check-cores)
and the guard's `ChangeHistory` row in
[ports and adapters](../../../docs/built-in-checks.md#ports-and-adapters).

## Behaviours to test

- A change to a path in `protected`, and to any `continual.toml`, is reported, untracked files
  included.
- A missing base revision makes the check could-not-run, never "no findings".
- Locally the guard passes with its findings; given the CI inputs and a list of labels, it
  fails unless the list holds the approval label.
- The approval label is `guard-approved`: the guard core defines the name once, and no
  setting changes it.
- The fake passes the `ChangeHistory` contract suite for the merge base, changed and deleted
  files, and a file's text at a revision and in the working tree.

## Done when

- The guard's own `ChangeHistory` is declared here, documented per P5.
- `arch.toml` classifies the new core, which depends on no other core or adapter and carries
  a `clippy.toml` of its own.
