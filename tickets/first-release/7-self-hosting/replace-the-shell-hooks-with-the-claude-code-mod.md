---
type: ticket
title: Replace the shell hooks with the Claude Code mod
parent: tickets/first-release/7-self-hosting/story
points: 3
priority: medium
status: pending
---

# Replace the shell hooks with the Claude Code mod

## Goal

Retire `.claude/hooks/*.sh` in favour of Continual's own harness mod, installed with
`continual install claude` and configured in this repository's `continual.toml`, so this
repository's agents work under the same review gate Continual ships.

## Design

The planned paragraph in
[Workflow: Claude Code settings and hooks](../../../docs/workflow.md#claude-code-settings-and-hooks)
and [Harness](../../../docs/harness.md).

## Behaviours to test

- Every behaviour the `xtask` hook tests check today holds with the mod: bypasses blocked,
  waves, pull requests asking inside a wave, and every form of commit, push and pull request
  asking.
- The edit and stop hooks run the steps `[events]` maps.
- `jq` is no longer needed.

## Done when

- `.claude/hooks/` no longer holds the shell scripts, and docs/workflow.md, CLAUDE.md and
  AGENTS.md describe the mod.
- The planned marker in docs/workflow.md is gone.
