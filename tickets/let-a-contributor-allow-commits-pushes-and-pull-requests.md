---
type: ticket
title: Let a contributor allow commits, pushes and pull requests
points: 3
priority: medium
status: pending
---

# Let a contributor allow commits, pushes and pull requests

## Goal

Keep the review gate by default, but let a contributor explicitly allow an agent to commit,
push or open pull requests without asking each time, as the
[harness mods](../VISION.md#harness-modsplugins) describe ("unless explicitly allowed").

## Design

[Workflow: allowances](../docs/workflow.md#allowances), the allowance paragraph in
[Claude Code settings and hooks](../docs/workflow.md#claude-code-settings-and-hooks), and
[Workflow: open questions](../docs/workflow.md#open-questions).

## Behaviours to test

- `/allow commit` lets commits through without asking, across later contributor messages.
- Each step is allowed on its own: an allowance for commits does not cover pushes, and one
  for pushes does not cover pull requests.
- `/allow pr` lets pull-request creation through, and a merge still asks.
- `/allow off` withdraws the allowance, and the next commit asks again.
- Inside an allowance, the commands the review gate always blocks stay blocked.
- An allowance in one session does not reach another session.
- `/allow` in a command the agent runs, or anywhere but a contributor's message, gives no
  allowance.
- Existing: without an allowance, the first commit or push after each message asks, waves
  open and close as before, and the existing hook tests pass unchanged.

## Done when

- The open question in docs/workflow.md is decided and removed.
- R1 in CONVENTIONS.md, the review gate in AGENTS.md and the hook notes in CLAUDE.md
  describe the allowance.
- The design documents describe the change, and its planned markers are gone.
