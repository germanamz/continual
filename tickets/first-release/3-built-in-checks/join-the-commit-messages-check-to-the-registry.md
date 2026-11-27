---
type: ticket
title: Join the commit-messages check to the registry
blocks: tickets/first-release/4-events/give-check-steps-an-events-files-and-message
parent: tickets/first-release/3-built-in-checks/story
points: 2
priority: medium
status: pending
---

# Join the commit-messages check to the registry

## Goal

Create the `continual-registry-commit-messages` bridge and wire it, so a check step runs the
commit-messages check with the repository's patterns on the messages CI gives it.

## Design

[Built-in checks: commit messages](../../../docs/built-in-checks.md#commit-messages) and
[bridges](../../../docs/built-in-checks.md#bridges).

## Behaviours to test

- The bridge states the name `commit-messages` and that the check is a repository check.
- `attribution`, `forbidden` and `required` are read into the core's settings, and an unknown
  key or a value of the wrong type is rejected with its line.
- A pattern that is not a valid regular expression is a configuration error at its line.
- The run's commit range, and the pull request's title and body, reach the core.
- The bridge passes the registry's `Check` contract suite.
- The real binary runs a `commit-messages` check step on a fixture repository.

## Done when

- The composition root builds the execution context, the commit-messages core with it, and
  the bridge, and hands the bridge to the registry.
- The bridge depends on the registry core and the commit-messages core and on no other
  core or adapter, and `arch.toml` classifies it.
- The design documents describe the change, and the planned markers of
  [Run the built-in checks in Continual](story.md) are gone once its other children are
  completed.
