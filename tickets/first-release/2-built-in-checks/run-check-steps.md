---
type: ticket
title: Run check steps
blocks:
  - tickets/first-release/2-built-in-checks/guard-protected-files-in-continual
  - tickets/first-release/2-built-in-checks/check-architecture-in-continual
  - tickets/first-release/2-built-in-checks/split-commit-messages-into-parts
  - tickets/first-release/5-packages/route-files-to-their-packages
parent: tickets/first-release/2-built-in-checks/story
points: 3
priority: high
status: pending
---

# Run check steps

## Goal

Let a step run built-in checks: read `checks` entries with their inline settings, run them in
list order within the pipeline, and print their findings as `path:line: message`, starting
with the comments check.

## Design

[Built-in checks: check steps](../../../docs/built-in-checks.md#check-steps) and
[core](../../../docs/built-in-checks.md#core).

## Behaviours to test

- A check step runs every check in list order, even after one reports findings, and fails
  when any check reports findings.
- A check that cannot run makes its step could-not-run (exit code 2), and never reports "no
  findings" instead.
- A per-file check inspects the event's files in an event and every tracked file of its
  package otherwise; a repository check ignores the event's files.
- An unknown check name, or an unknown setting of a check, is a configuration error at its
  line.
- Findings print with paths relative to the repository root.
- The real binary runs a `comments` check step on a fixture repository.

## Done when

- The composition root wires `continual-rust` and `continual-git` into check steps.
