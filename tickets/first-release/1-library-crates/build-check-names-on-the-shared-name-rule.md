---
type: ticket
title: Build check names on the shared name rule
parent: tickets/first-release/1-library-crates/story
points: 1
priority: medium
status: pending
---

# Build check names on the shared name rule

## Goal

Build the registry's `CheckName` on the library crate's `Name`, so the registry core holds no
copy of the name rule and the last duplicate of it is gone (Q16).

## Design

The name rule in [Workspace: library crates](../../../docs/workspace.md#library-crates) and the
decision "One name rule, and a type for each kind of name" in
[decisions](../../../docs/workspace.md#decisions).

## Carried over

- As for step names, the registry's tests of the rule itself are covered by the library
  crate's, and its message assertions change with the one error enum: changes to existing
  tests (G5) that the decision calls for, reported by the guard and told to the contributor.
- `CheckName` keeps printing as the text it was parsed from, and keeps `str::parse`.

## Behaviours to test

- `CheckName::parse` and `str::parse` accept and reject through the shared rule, with
  `NameError`.
- A `CheckName` prints as the text it was parsed from.

## Done when

- The registry holds no copy of the rule, and a `CheckName` is still a type of its own.
- The doc comments and the design documents no longer say that one kind of name follows
  another's rules.
