---
type: ticket
title: Join the comments check to the registry
blocks: tickets/first-release/2-built-in-checks/run-check-steps
parent: tickets/first-release/2-built-in-checks/story
points: 2
priority: high
status: pending
---

# Join the comments check to the registry

## Goal

Create the `continual-registry-comments` bridge, which implements the registry's `Check` by
calling the comments core, so the registry runs the comments check without either core
knowing the other.

## Design

[Built-in checks: bridges](../../../docs/built-in-checks.md#bridges), and the bridge rules H2
and H3 in [CONVENTIONS.md](../../../CONVENTIONS.md#cores-h).

## Behaviours to test

- The bridge states the name `comments` and that the check is a per-file check.
- The comments check has no settings, so any key beside its name is rejected with its line.
- A file's path and text reach the comments core, and each finding comes back with its path,
  line and message.
- A file the comments core answers as not read is answered the same way to the registry.
- The bridge passes the registry's `Check` contract suite.

## Done when

- The bridge depends on the registry core and the comments core and on no other workspace
  crate, and `arch.toml` classifies it.
