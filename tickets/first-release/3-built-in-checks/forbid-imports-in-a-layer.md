---
type: ticket
title: Forbid imports in a layer
blocks:
  - tickets/first-release/3-built-in-checks/read-rust-imports
  - tickets/first-release/3-built-in-checks/load-units-from-the-source-tree
parent: tickets/first-release/3-built-in-checks/story
points: 2
priority: medium
status: pending
---

# Forbid imports in a layer

## Goal

Add the `forbidden_imports` rule to the architecture core, over the imports a language
adapter reports, with the imports operation added to its `LanguageSyntax`.

## Design

The `forbidden_imports` bullet in [Built-in checks: architecture](../../../docs/built-in-checks.md#architecture).

## Behaviours to test

- An import path a layer forbids, used in one of the layer's source files, is reported at its
  line.
- A forbidden path matches the path and anything under it, so `std::fs` covers
  `std::fs::read`.
- The fake passes the `LanguageSyntax` contract suite for imports.

## Done when

- The rule is tested in the architecture core with the fake.
