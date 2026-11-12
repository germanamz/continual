---
type: ticket
title: Recognise gated actions
blocks:
  - tickets/first-release/4-claude-code-mod/block-commands-that-skip-checks
  - tickets/first-release/4-claude-code-mod/open-and-close-review-waves
parent: tickets/first-release/4-claude-code-mod/story
points: 3
priority: medium
status: pending
---

# Recognise gated actions

## Goal

Give the core the `gate` module's first slice: from a command's segments of words, recognise a
commit, a push, a pull-request creation or a merge, and answer "ask" for the actions in
`[review_gate] ask`.

## Design

[Harness: the review gate](../../../docs/harness.md#the-review-gate) and the core notes in
[Harness: design](../../../docs/harness.md#design).

## Behaviours to test

- Commits, pushes, `gh pr create` and `gh pr merge` are recognised, including through
  `git -C <dir>` and a full path to the program.
- In a chain of segments, any gated segment makes the command gated.
- A recogniser in `[review_gate.commands]` adds a tool, such as `glab mr create`.
- A gated action listed in `ask` is answered "ask"; one left out of `ask` is allowed; the
  default `ask` lists all four.
- An unknown gated action in `ask` is a configuration error.
- Ordinary commands are allowed.

## Done when

- The rules are tested in the core with segment inputs; no shell parsing is in the core.
