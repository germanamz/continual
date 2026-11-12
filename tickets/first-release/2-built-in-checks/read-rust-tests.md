---
type: ticket
title: Read Rust tests
blocks: tickets/first-release/2-built-in-checks/read-rust-lint-suppressions
parent: tickets/first-release/2-built-in-checks/story
points: 3
priority: medium
status: pending
---

# Read Rust tests

## Goal

Implement the tests operation of `LanguageSyntax` in `continual-rust` with `syn`, so the
guard's test rules work on Rust as the `xtask` guard does today.

## Design

The `rust` notes in [Built-in checks: ports and adapters](../../../docs/built-in-checks.md#ports-and-adapters).

## Behaviours to test

- The `LanguageSyntax` contract suite for tests passes against the adapter.
- `#[test]` functions are reported with their names, attributes (`#[ignore]`,
  `#[should_panic]`) and a token fingerprint that reformatting does not change.
- Helpers and attributes in `tests/` files and `#[cfg(test)]` modules are reported.
- Each case the `xtask` guard tests cover for tests gives the same findings.

## Done when

- Existing: the `xtask` guard keeps passing its tests unchanged.
