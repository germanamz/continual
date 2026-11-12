---
type: ticket
title: Read staged and uncommitted files in git
blocks: tickets/first-release/3-events/run-an-event-from-the-command-line
parent: tickets/first-release/3-events/story
points: 2
priority: medium
status: pending
---

# Read staged and uncommitted files in git

## Goal

Implement the staged and uncommitted files of `ChangeHistory` in the `continual-git` adapter,
with git's prompts turned off and a time limit, so reading them never hangs.

## Design

The `git` notes in [Built-in checks: ports and adapters](../../../docs/built-in-checks.md#ports-and-adapters)
and [Pipeline: timeouts](../../../docs/pipeline.md#timeouts).

## Behaviours to test

- The `ChangeHistory` contract suite for the staged and uncommitted files passes against the
  adapter in fixture repositories.
- Untracked files count as uncommitted; ignored files do not.
- A git command that would prompt, or runs past the adapter's limit, is reported as an error.

## Done when

- `continual-git` is created here if it does not exist yet, and `arch.toml` classifies it.
