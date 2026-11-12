---
type: ticket
title: Inherit shared steps in packages
blocks: tickets/first-release/5-packages/route-files-to-their-packages
parent: tickets/first-release/5-packages/story
points: 3
priority: medium
status: pending
---

# Inherit shared steps in packages

## Goal

Give every package the root's `[workspace.steps]`, let a package replace one by defining a
step of the same name, and let a step depend on another package's step, all validated in the
core's `Workspace`.

## Design

[Pipeline: packages](../../../docs/pipeline.md#packages) and the `workspace` module in
[Pipeline: core](../../../docs/pipeline.md#core).

## Behaviours to test

- Every package has every `[workspace.steps]` step; a step the package defines under the same
  name replaces it whole, with no field merged.
- `[workspace.steps]` never become root steps; the root's `[steps]` belong to the root alone.
- `shared:build` in `depends_on` resolves to the `build` step of the package named `shared`;
  an unknown package or step is a problem naming both.
- A cycle across packages is rejected, naming its steps with their packages.

## Done when

- The rules are tested in the core through the public API.
