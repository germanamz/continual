---
type: ticket
title: Judge with the base revision's guard settings
blocks: tickets/first-release/3-built-in-checks/run-the-guard-on-its-own
parent: tickets/first-release/3-built-in-checks/story
points: 3
priority: medium
status: pending
---

# Judge with the base revision's guard settings

## Goal

Make the guard judge with the `protected` list of the base revision's
`continual.toml`, so a change cannot loosen the guard that judges it: a check says where its
settings come from, and the registry asks its new `BaseEntries` port, which the
`engine-registry` bridge answers from the engine, for the entries of one that judges with the
base revision's.

## Design

The base-revision paragraph in [Built-in checks: guard](../../../docs/built-in-checks.md#guard),
[the registry core](../../../docs/built-in-checks.md#the-registry-core), the `BaseEntries` row
in [ports and adapters](../../../docs/built-in-checks.md#ports-and-adapters),
[bridges](../../../docs/built-in-checks.md#bridges) and the decision in
[decisions](../../../docs/built-in-checks.md#decisions).

## Behaviours to test

- A check that judges with the base revision's settings runs once with each entry
  `BaseEntries` gives for the step's package, not with the step's own settings.
- A change that removes a path from `protected` is still reported for that path.
- A package with no guard entry at the base revision, and a base revision with no
  `continual.toml`, are judged with the built-in settings: `continual.toml` as the only
  protected path. The change that adds the first guard entry is
  reported for its `continual.toml`.
- `BaseEntries` is documented per P5, and the fake and the bridge pass its contract suite.
- The real binary still reports a change that removes a path from `protected`.

## Done when

- The rules are tested in the registry core with a fake check and the `BaseEntries` fake, and
  the guard's bridge says that the guard judges with the base revision's settings.
- The composition root builds the engine's reader of base entries before the registry, and
  the bridge only maps between the two cores' types.
