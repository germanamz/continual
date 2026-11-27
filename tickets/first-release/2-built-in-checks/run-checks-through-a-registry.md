---
type: ticket
title: Run checks through a registry
blocks:
  - tickets/first-release/2-built-in-checks/inspect-every-tracked-file
  - tickets/first-release/2-built-in-checks/join-the-comments-check-to-the-registry
  - tickets/first-release/2-built-in-checks/join-the-engine-to-the-registry
parent: tickets/first-release/2-built-in-checks/story
points: 3
priority: high
status: pending
---

# Run checks through a registry

## Goal

Make the registry core the check registry: it holds checks by name behind its `Check` port
and runs a step's check entries through them, so the code that runs checks names none of them
and a new check changes nothing in it.

## Design

[Built-in checks: the registry core](../../../docs/built-in-checks.md#the-registry-core), the
`Check` row in [ports and adapters](../../../docs/built-in-checks.md#ports-and-adapters) and
the registry's decision in [decisions](../../../docs/built-in-checks.md#decisions).

## Behaviours to test

- The registry finds a check by its name, a name it does not hold is an error, and two checks
  with one name are refused.
- A check reads its own settings from located values, and what it rejects comes back with its
  line.
- With fake checks, the registry runs every entry of a step in list order, even after one
  reports findings, and labels each finding with its check's name.
- A check that could not run makes the step could-not-run, never "no findings".
- A check's findings fail the step unless the check says they only warn.
- The run's inputs reach every repository check.
- `Check` is documented per P5, and the fake passes its contract suite.

## Done when

- The registry core names no check and is tested with fake checks only.
