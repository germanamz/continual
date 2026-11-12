---
type: ticket
title: Self-host Continual on its first release
priority: high
status: pending
---

# Self-host Continual on its first release

## Goal

Ship Continual's first release, with the pipeline, packages, events, the built-in checks and
the Claude Code mod, and make that pinned release the gate of Continual's own repository in
place of `cargo xtask` ([VISION.md](../../VISION.md)). This is the epic that holds every feature
ticket of that milestone.

## Design

- [Pipeline](../../docs/pipeline.md), [Built-in checks](../../docs/built-in-checks.md),
  [Harness](../../docs/harness.md) and [Distribution](../../docs/distribution.md).
- [Checks: self-hosting](../../docs/checks.md#self-hosting).

## Behaviours to test

Each feature ticket under this epic, and each task under those, lists the behaviours it adds.
List the children with the query in [tickets/README.md](../README.md#with-tusk).

## Done when

- Every feature ticket under this epic is completed.
- A released `continual`, pinned in this repository's `continual.toml`, gates this repository
  locally and in CI.
