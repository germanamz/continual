---
type: ticket
title: Route files to their packages
blocks: tickets/first-release/5-packages/run-a-pipeline-in-every-package
parent: tickets/first-release/5-packages/story
points: 2
priority: medium
status: pending
---

# Route files to their packages

## Goal

Make every file belong to exactly one package, so an event's files and a per-file check's
files go to the package that owns them and no file is checked twice.

## Design

The file-ownership bullet in [Pipeline: packages](../../../docs/pipeline.md#packages) and the
package rules in [Pipeline: events](../../../docs/pipeline.md#events).

## Behaviours to test

- A file belongs to the deepest package that contains it, and the root owns every file outside
  the listed packages.
- An event runs a package's pipelines only when the package owns one of its files, with only
  those files in its `{files}`; the root's pipelines run on every event.
- A per-file check in a root step inspects only the files the root owns.

## Done when

- The rules are tested in the core with fakes.
