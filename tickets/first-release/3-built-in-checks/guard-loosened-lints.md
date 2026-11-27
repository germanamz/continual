---
type: ticket
title: Guard loosened lints
blocks: tickets/first-release/3-built-in-checks/read-rust-lint-suppressions
parent: tickets/first-release/3-built-in-checks/story
points: 2
priority: medium
status: pending
---

# Guard loosened lints

## Goal

Add the lint rules to the guard core: a new lint suppression, or a change that loosens the
language's lint configuration, needs approval. The suppressions operation is added to its
`LanguageSyntax`.

## Design

The lint bullet in [Built-in checks: guard](../../../docs/built-in-checks.md#guard).

## Behaviours to test

- A suppression that is not in the base revision is reported at its line.
- A lint configuration the language reports as loosened is reported.
- A suppression that already existed is not reported.
- The fake passes the `LanguageSyntax` contract suite for suppressions.

## Done when

- The rules are tested in the guard core with the fake.
