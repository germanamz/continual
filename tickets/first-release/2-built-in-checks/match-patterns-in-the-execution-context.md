---
type: ticket
title: Match patterns in the execution context
blocks: tickets/first-release/2-built-in-checks/join-the-commit-messages-check-to-the-registry
parent: tickets/first-release/2-built-in-checks/story
points: 2
priority: medium
status: pending
---

# Match patterns in the execution context

## Goal

Implement the commit-messages core's `Patterns` in the `continual-context` adapter, the
execution context, with regular expressions, so the check matches a repository's patterns
while no core depends on a library.

## Design

The `context` note and the `Patterns` row in
[Built-in checks: ports and adapters](../../../docs/built-in-checks.md#ports-and-adapters), the
pattern decision in [decisions](../../../docs/built-in-checks.md#decisions), and the crate in
[Workspace: planned crates](../../../docs/workspace.md#planned-crates).

## Behaviours to test

- The `Patterns` contract suite passes against the adapter.
- A pattern such as `(?i)^wip\b` matches as the `regex` crate reads it, and text that is not
  a valid regular expression is rejected with the reason.
- No type or error of the `regex` crate crosses the port (A4).

## Done when

- The adapter depends on the cores it answers, now the commit-messages core among them, and
  on no other workspace crate.
- `regex` is the crate's dependency, added per G8 with its reason, and every core's
  allow-list stays empty.
