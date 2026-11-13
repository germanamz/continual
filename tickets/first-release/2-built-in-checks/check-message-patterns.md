---
type: ticket
title: Check message patterns
blocks: tickets/first-release/2-built-in-checks/check-commit-messages-in-continual
parent: tickets/first-release/2-built-in-checks/story
points: 3
priority: medium
status: pending
---

# Check message patterns

## Goal

Add a repository's own `forbidden` and `required` regular expressions to the commit-messages
check, each applied to one part of the message and reported with its reason.

## Design

[Built-in checks: commit messages](../../../docs/built-in-checks.md#commit-messages) and the regex
decision in [decisions](../../../docs/built-in-checks.md#decisions).

## Behaviours to test

- A `forbidden` pattern that matches its part is reported with the pattern's `reason`.
- A `required` pattern that does not match its part is reported with its `reason`.
- Each pattern applies only to its `part`.
- An invalid regular expression is a configuration error at its line.

## Done when

- `regex` is on `arch.toml`'s core allow-list with its reason, after the contributor approves
  it.
