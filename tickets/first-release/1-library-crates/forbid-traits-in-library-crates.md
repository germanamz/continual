---
type: ticket
title: Forbid traits in library crates
parent: tickets/first-release/1-library-crates/story
points: 1
priority: medium
status: pending
---

# Forbid traits in library crates

## Goal

Make `cargo xtask arch` report a trait declared in a library crate's sources, so no port can
live in one and ports stay in the cores (H6, P7).

## Design

The `arch` bullet in [Checks: library crates](../../../docs/checks.md#library-crates) and the
decision "A library crate declares no trait" in
[decisions](../../../docs/checks.md#decisions).

## Behaviours to test

- A `trait` item in a library crate's source file is reported at its line, one inside an
  inline module included.
- A trait that a library crate implements, such as `Display`, is not reported.
- A trait declared in a core is not reported.

## Done when

- Fixture tests show the scan failing and passing.
- This task's planned marker is gone from `docs/checks.md`, and H6's "Check" cell in
  `CONVENTIONS.md` names the scan (a guarded change).
