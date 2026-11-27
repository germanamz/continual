---
type: ticket
title: Check message patterns
blocks:
  - tickets/first-release/2-built-in-checks/check-commit-messages-in-continual
  - tickets/first-release/2-built-in-checks/match-patterns-in-the-execution-context
parent: tickets/first-release/2-built-in-checks/story
points: 3
priority: medium
status: pending
---

# Check message patterns

## Goal

Add a repository's own `forbidden` and `required` patterns to the commit-messages core, each
applied to one part of the message and reported with its reason. The core reads and matches
them through its new `Patterns` port, so it gains no dependency.

## Design

[Built-in checks: commit messages](../../../docs/built-in-checks.md#commit-messages), the
`Patterns` row in [ports and adapters](../../../docs/built-in-checks.md#ports-and-adapters) and
the pattern decision in [decisions](../../../docs/built-in-checks.md#decisions).

## Behaviours to test

- A `forbidden` pattern that matches its part is reported with the pattern's `reason`.
- A `required` pattern that does not match its part is reported with its `reason`.
- Each pattern applies only to its `part`.
- A pattern that `Patterns` says is not valid is rejected when the settings are built, naming
  the pattern, so that the check's bridge can give its line.
- `Patterns` is documented per P5, and the fake passes its contract suite.

## Done when

- The rules are tested in the commit-messages core with the fake.
- The contract suite and the fake live in the core's `testing` module, and the core's
  allow-list in `arch.toml` stays empty.
