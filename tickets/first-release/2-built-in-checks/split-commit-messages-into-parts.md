---
type: ticket
title: Split commit messages into parts
blocks: tickets/first-release/2-built-in-checks/reject-ai-attribution-in-messages
parent: tickets/first-release/2-built-in-checks/story
points: 1
priority: medium
status: pending
---

# Split commit messages into parts

## Goal

Give the core a commit message value that splits into the subject, the body, the trailers and
the whole message, which every message rule reads.

## Design

The `part` bullet in [Built-in checks: commit messages](../../../docs/built-in-checks.md#commit-messages).

## Behaviours to test

- The subject is the first line; the body is the lines after the first blank line, trailers
  excluded; the trailers are the last paragraph's `Key: value` lines.
- A message with CRLF line endings splits into the same parts as with LF.
- A pull-request title is a subject and its description a body.

## Done when

- The value is tested in the core through the public API.
