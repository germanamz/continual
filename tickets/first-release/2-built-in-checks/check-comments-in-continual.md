---
type: ticket
title: Check comments in Continual
blocks:
  - tickets/first-release/2-built-in-checks/read-rust-comments
  - tickets/first-release/2-built-in-checks/join-the-comments-check-to-the-registry
parent: tickets/first-release/2-built-in-checks/story
points: 3
priority: high
status: pending
---

# Check comments in Continual

## Goal

Create the comments core, `continual-comments`, with the comments rule written once over the
comment facts a language adapter reports, and its `LanguageSyntax` port.

## Design

[Built-in checks: comments](../../../docs/built-in-checks.md#comments),
[the check cores](../../../docs/built-in-checks.md#the-check-cores) and the comments row in
[ports and adapters](../../../docs/built-in-checks.md#ports-and-adapters).

## Behaviours to test

- Two or more consecutive lines holding only a non-doc line comment are reported once, with
  their length.
- Any block comment is reported, a nested one counting once.
- A line comment the language marks as code is reported.
- Doc comments are never reported.
- A file whose extension no language adapter states is answered as not read, never as clean.
- `LanguageSyntax` is documented per P5, and the fake passes its contract suite.

## Done when

- The rule is tested in the comments core with the fake; no language parser is in it.
- `arch.toml` classifies the new core, which depends on no other workspace crate and carries
  a `clippy.toml` of its own.
