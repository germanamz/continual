---
type: ticket
title: Run a pipeline in every package
parent: tickets/first-release/5-packages/story
points: 3
priority: medium
status: pending
---

# Run a pipeline in every package

## Goal

Make `continual run` understand packages: a bare pipeline name at the root runs in every
package that has it and in the root, inside a package it runs that package's pipeline, and
`web:check` names one package from anywhere, with each tool running in its package's
directory.

## Design

The run rules at the end of [Pipeline: packages](../../../docs/pipeline.md#packages).

## Behaviours to test

- From the root, `continual run check` runs `check` in every package that has it, and in the
  root if the root defines it.
- From inside a package directory, `continual run check` runs only that package's `check` and
  the pipelines it depends on, in whichever packages they are.
- `continual run web:check` runs one package's pipeline from any directory.
- A package's tool runs in that package's directory; the root's own pipelines run once, in the
  root directory; shared pipelines never run at the root.
- Each prefixed output line, failure line and summary line names the entry's package.

## Done when

- A fixture monorepo with two packages and a cross-package pipeline dependency runs in the
  right order through the real binary.
- The design documents describe the change, and the planned markers of
  [Run steps across a repository's packages](story.md) are gone.
