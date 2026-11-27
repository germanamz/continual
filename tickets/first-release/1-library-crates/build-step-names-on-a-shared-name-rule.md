---
type: ticket
title: Build step names on a shared name rule
blocks: tickets/first-release/1-library-crates/build-check-names-on-the-shared-name-rule
parent: tickets/first-release/1-library-crates/story
points: 2
priority: medium
status: completed
---

# Build step names on a shared name rule

## Goal

Move the name rule into `continual-vocabulary` as the value object `Name`, with its one
error enum, and build the engine's `StepName` on it, so the rule a step name and a check
name both follow has one definition (Q16).

## Design

The name rule in [Workspace: library crates](../../../docs/workspace.md#library-crates) and the
decision "One name rule, and a type for each kind of name" in
[decisions](../../../docs/workspace.md#decisions).

## Carried over

- The contributor chose one rule with one error enum, whose messages name no kind of name.
  The engine's tests of the rule therefore move to the library crate and its message
  assertions change. These are changes to existing tests (G5) that the decision calls for:
  the guard still reports them, and the contributor is told of each when the change is ready.
- Every name the engine holds by then is built on `Name`: step names, and pipeline or package
  names if their tasks are merged.
- A configuration problem for an invalid step name still names the step and the rule it
  breaks.
- `Name` joins the domain glossary in `CONVENTIONS.md` (a guarded change).
- The engine re-exports `Location` and `Located<T>` from its `workspace` module, so each has
  two paths. Whether it re-exports `Name`'s error as well is open in
  [Workspace: open questions](../../../docs/workspace.md#open-questions). Settle it with the
  contributor before the error gets a second path.

## Behaviours to test

- `Name::parse` accepts and rejects every case `StepName::parse` does today, with the same
  error and the same position.
- A `NameError` explains itself without naming a step or a check.
- `StepName::parse` accepts and rejects through the shared rule.
- An invalid step name is still reported at the step's location, with the name as written.

## Done when

- The engine holds no copy of the rule, and a `StepName` is still a type of its own that no
  other kind of name can stand in for.
- `Name` carries no planned link in `docs/workspace.md`.
