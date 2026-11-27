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
- `CheckName` keeps printing as the text it was parsed from, and keeps `str::parse`. `Name`
  has `parse` and `as_str` and neither `Display` nor `FromStr`, so `CheckName` implements
  both itself over those two.
- `crates/libraries/vocabulary/tests/name.rs` holds every case of the rule. The engine kept
  two tests of `StepName::parse`, one that accepts and one that rejects, and moved the rest.
- `NameError` says "name" where `CheckNameError` says "check name", such as `name is empty`.
  Whoever reports one says which kind of name it is, as the engine's
  `ProblemKind::InvalidStepName` prints `invalid step name "Fmt": name starts with 'F', not
  a lowercase letter`. Nothing reports a check name's error yet, so the registry adds no
  such wrapper before a use needs one.
- The engine does not re-export `Name` or `NameError`, and its tests name
  `continual_vocabulary::NameError`, so the error has one path. Whether a core re-exports
  library types is still open in
  [Workspace: open questions](../../../docs/workspace.md#open-questions). Unless the
  contributor settles it otherwise first, the registry names the error by the library
  crate's path too.
- What still says that one kind of name follows another's rules: `CheckName`'s doc comment,
  which states the rule itself, the `[steps.<name>]` bullet in
  [Pipeline: `continual.toml`](../../../docs/pipeline.md#continualtoml), and the behaviours
  of the open tickets that say a name "follows `CheckName`'s rules". The glossary in
  `CONVENTIONS.md` has `Name` to point them at.
- `docs/workspace.md` marks `CheckName` as planned under "Each kind of name stays a type of
  its own", and says in its Behaviour section that the registry exposes `CheckNameError`.

## Behaviours to test

- `CheckName::parse` and `str::parse` accept and reject through the shared rule, with
  `NameError`.
- A `CheckName` prints as the text it was parsed from.

## Done when

- The registry holds no copy of the rule, and a `CheckName` is still a type of its own.
- The doc comments and the design documents no longer say that one kind of name follows
  another's rules.
