---
type: ticket
title: Inherit shared steps and pipelines in packages
blocks: tickets/first-release/5-packages/route-files-to-their-packages
parent: tickets/first-release/5-packages/story
points: 3
priority: medium
status: pending
---

# Inherit shared steps and pipelines in packages

## Goal

Give every package the root's `[workspace.steps]` and `[workspace.pipelines]`, let a package
replace either by defining one of the same name, and let a pipeline depend on another
package's pipeline, all validated in the core's `Workspace`.

## Design

[Pipeline: packages](../../../docs/pipeline.md#packages) and the `workspace` module in
[Pipeline: core](../../../docs/pipeline.md#core).

## Behaviours to test

- Every package has every shared step and pipeline; one the package defines under the same
  name replaces it whole, with no field merged.
- A package that replaces a shared pipeline with one that leaves a shared step out never runs
  that step.
- Shared definitions never become the root's own; the root's `[steps]` and `[pipelines]`
  belong to the root alone.
- `shared:build` in a pipeline's `depends_on` resolves to the `build` pipeline of the package
  named `shared`; an unknown package or pipeline is a problem naming both.
- A cycle among pipelines across packages is rejected, naming them with their packages.

## Done when

- The rules are tested in the core through the public API.
