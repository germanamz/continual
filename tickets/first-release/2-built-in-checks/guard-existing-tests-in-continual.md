---
type: ticket
title: Guard existing tests in Continual
blocks:
  - tickets/first-release/2-built-in-checks/read-rust-tests
  - tickets/first-release/2-built-in-checks/guard-loosened-lints
parent: tickets/first-release/2-built-in-checks/story
points: 3
priority: medium
status: pending
---

# Guard existing tests in Continual

## Goal

Add the guard's test rules in the core, over the test items a language adapter reports with
their token fingerprints, with the tests operation added to `LanguageSyntax`.

## Design

[Built-in checks: guard](../../../docs/built-in-checks.md#guard) and the fingerprint note in
[ports and adapters](../../../docs/built-in-checks.md#ports-and-adapters).

## Behaviours to test

- An existing test deleted, renamed, ignored, made to expect a failure, or changed in its
  fingerprint is reported; one whose fingerprint is unchanged is not.
- A deleted file that held tests is reported.
- A changed helper or file or module attribute in a test file or test module is reported.
- Adding a test is never reported.
- A source file the adapter cannot parse is reported, not skipped.
- The fake passes the `LanguageSyntax` contract suite for tests.

## Done when

- The rules are tested in the core with the fake, without knowing any language.
