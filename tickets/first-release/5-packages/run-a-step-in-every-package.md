---
type: ticket
title: Run a step in every package
parent: tickets/first-release/5-packages/story
points: 3
priority: medium
status: pending
---

# Run a step in every package

## Goal

Make `continual run` understand packages: a bare step name at the root runs in every package
that has it and in the root, inside a package it runs that package's step, and `web:test`
names one package from anywhere, with each tool running in its package's directory.

## Design

The run rules at the end of [Pipeline: packages](../../../docs/pipeline.md#packages).

## Behaviours to test

- From the root, `continual run test` runs `test` in every package that has it, and in the
  root if the root defines it.
- From inside a package directory, `continual run test` runs only that package's `test` and
  its upstream steps, in whichever packages they are.
- `continual run web:test` runs one package's step from any directory.
- A package's tool runs in that package's directory; the root's `[steps]` run once, in the
  root directory; `[workspace.steps]` never run at the root.
- Each start, outcome and summary line names the step's package.

## Done when

- A fixture monorepo with two packages and a cross-package dependency runs in the right order
  through the real binary.
- The design documents describe the change, and the planned markers of
  [Run steps across a repository's packages](story.md) are gone.
