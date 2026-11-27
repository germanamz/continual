---
type: ticket
title: Read check entries at the base revision
blocks: tickets/first-release/2-built-in-checks/judge-with-the-base-guard-settings
parent: tickets/first-release/2-built-in-checks/story
points: 3
priority: medium
status: pending
---

# Read check entries at the base revision

## Goal

Give the engine the operation that reads the configuration at the merge base with a revision
and gives one check's entries in it, so the guard can be judged with settings the change
cannot touch.

## Design

The engine's answers to other cores in
[Pipeline: the engine core](../../../docs/pipeline.md#the-engine-core), and the `ConfigSource` and
`ChangeHistory` rows in [Pipeline: ports and adapters](../../../docs/pipeline.md#ports-and-adapters).

## Behaviours to test

- The engine reads every `continual.toml` at the merge base through `ChangeHistory`, and
  `ConfigSource` turns each one's text into configuration input.
- The entries of the named check are given, each with its package and its settings as located
  values; a base revision with no `continual.toml`, or with no such entry, gives none.
- The operation builds no `Workspace`, asks no `CheckRunner` and ignores the version the base
  revision pins.
- A base configuration that cannot be read, and a base revision missing from a shallow clone,
  are each reported as an error naming it.
- The contract suites for the new `ChangeHistory` and `ConfigSource` operations pass against
  the fakes, `continual-git` and `continual-toml-config`.

## Done when

- The operation is tested in the engine core with fakes.
- The operations are added to the engine's own `ChangeHistory` and to `ConfigSource`,
  documented per P5; `ChangeHistory` is created here if it does not exist yet.
