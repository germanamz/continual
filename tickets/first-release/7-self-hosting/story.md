---
type: ticket
title: Check Continual with a pinned release
order: 7
parent: tickets/first-release/epic
priority: medium
status: pending
---

# Check Continual with a pinned release

## Goal

Make Continual check its own repository: a root `continual.toml` replaces `cargo xtask ci`,
the pinned release is the gate locally and in CI, a dogfood job runs the working-tree build,
and the shell hooks give way to the harness mod ([pipeline](../../../VISION.md#pipeline),
[CI](../../../VISION.md#ci)).

## Design

[Checks: self-hosting](../../../docs/checks.md#self-hosting) and the planned paragraph in
[Workflow: Claude Code settings and hooks](../../../docs/workflow.md#claude-code-settings-and-hooks).

## Behaviours to test

Each child ticket lists the behaviours it adds. List the children with the query in
[tickets/README.md](../../README.md#with-tusk). Throughout, each `xtask` check is removed only once
the pinned release covers it.

## Done when

- Every child ticket is completed.
- `cargo xtask ci` is no longer the gate, and the moved checks are gone from `xtask`.
- The design documents describe the change, and its planned markers are gone.
