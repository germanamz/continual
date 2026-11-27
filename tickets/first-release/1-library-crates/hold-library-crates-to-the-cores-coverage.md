---
type: ticket
title: Hold library crates to the cores' coverage
parent: tickets/first-release/1-library-crates/story
points: 1
priority: medium
status: pending
---

# Hold library crates to the cores' coverage

## Goal

Make `cargo xtask coverage` hold each library crate's lines to the minimum a core's lines
meet, since every core builds on what a library crate holds.

## Design

The `coverage` bullet in [Checks: library crates](../../../docs/checks.md#library-crates) and
rule H7 in [CONVENTIONS.md](../../../CONVENTIONS.md#cores-h).

## Carried over

- The coverage paragraph under the T rules in `CONVENTIONS.md` and the `coverage` paragraph
  in `docs/checks.md` speak of each core's lines. This task adds the library crates to both;
  the first is a guarded change.

## Behaviours to test

- A library crate whose lines fall below the cores' minimum is a violation that names its
  crate.
- A library crate with no data in the report is a violation.
- A workspace without library crates is judged as it is today, and one without cores still
  fails.

## Done when

- The library crates come from `arch.toml` and `cargo metadata`, as the cores do.
- This task's planned marker is gone from `docs/checks.md`.
