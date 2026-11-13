---
type: ticket
title: Check comments in Continual
blocks:
  - tickets/first-release/2-built-in-checks/read-rust-comments
  - tickets/first-release/2-built-in-checks/check-comments-in-every-tracked-file
parent: tickets/first-release/2-built-in-checks/story
points: 3
priority: high
status: pending
---

# Check comments in Continual

## Goal

Write the comments rule once in the core, over the comment facts a language adapter reports,
and add the `LanguageSyntax` port with the comments operation it needs.

## Design

[Built-in checks: comments](../../../docs/built-in-checks.md#comments),
[core](../../../docs/built-in-checks.md#core) and the `LanguageSyntax` row in
[ports and adapters](../../../docs/built-in-checks.md#ports-and-adapters).

## Behaviours to test

- Two or more consecutive lines holding only a non-doc line comment are reported once, with
  their length.
- Any block comment is reported, a nested one counting once.
- A line comment the language marks as code is reported.
- Doc comments are never reported.
- `LanguageSyntax` is documented per P5, and the fake passes its contract suite for comments.

## Done when

- The rule is tested in the core with the fake; no language parser is in the core.
