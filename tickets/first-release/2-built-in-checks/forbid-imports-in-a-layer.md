---
type: ticket
title: Forbid imports in a layer
blocks: tickets/first-release/2-built-in-checks/read-rust-imports
parent: tickets/first-release/2-built-in-checks/story
points: 2
priority: medium
status: pending
---

# Forbid imports in a layer

## Goal

Add the architecture check's `forbidden-imports` rule in the core, over the imports a
language adapter reports, with the imports operation added to `LanguageSyntax`.

## Design

The `forbidden-imports` bullet in [Built-in checks: architecture](../../../docs/built-in-checks.md#architecture).

## Behaviours to test

- An import path a layer forbids, used in one of the layer's source files, is reported at its
  line.
- A forbidden path matches the path and anything under it, so `std::fs` covers
  `std::fs::read`.
- The fake passes the `LanguageSyntax` contract suite for imports.

## Done when

- The rule is tested in the core with the fake.
