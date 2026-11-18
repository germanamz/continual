---
type: ticket
title: Run steps across a repository's packages
blocks: tickets/first-release/6-releases/publish-the-first-release
order: 5
parent: tickets/first-release/epic
priority: medium
status: pending
---

# Run steps across a repository's packages

## Goal

Let a monorepo list its packages in the root `continual.toml`, let each package configuration
inherit the root's shared steps and pipelines, and let a pipeline depend on another package's
pipeline, so one command covers every package ([pipeline](../../../VISION.md#pipeline)).

## Design

[Pipeline: packages](../../../docs/pipeline.md#packages) and the `workspace` module in
[Pipeline: core](../../../docs/pipeline.md#core).

## Behaviours to test

Each child ticket lists the behaviours it adds. List the children with the query in
[tickets/README.md](../../README.md#with-tusk).

## Done when

- Every child ticket is completed.
- A fixture monorepo with two packages and a cross-package dependency runs in the right order.
- The design documents describe the change, and its planned markers are gone.
