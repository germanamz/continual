---
type: ticket
title: Split commit messages into parts
blocks: tickets/first-release/3-built-in-checks/reject-ai-attribution-in-messages
parent: tickets/first-release/3-built-in-checks/story
points: 2
priority: medium
status: pending
---

# Split commit messages into parts

## Goal

Create the commit-messages core, `continual-commit-messages`, with a commit message value that
splits into the subject, the body, the trailers and the whole message, which every message
rule reads.

## Design

The `part` bullet in [Built-in checks: commit messages](../../../docs/built-in-checks.md#commit-messages)
and [the check cores](../../../docs/built-in-checks.md#the-check-cores).

## Behaviours to test

- The subject is the first line; the body is the lines after the first blank line, trailers
  excluded; the trailers are the last paragraph's `Key: value` lines.
- A message with CRLF line endings splits into the same parts as with LF.
- A pull-request title is a subject and its description a body.

## Done when

- The value is tested in the commit-messages core through its public API.
- `arch.toml` classifies the new core, which depends on no other core or adapter and carries
  a `clippy.toml` of its own.
