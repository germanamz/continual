---
type: ticket
title: Recognise gated actions
blocks: tickets/first-release/4-claude-code-mod/block-commands-that-skip-checks
parent: tickets/first-release/4-claude-code-mod/story
points: 3
priority: medium
status: pending
---

# Recognise gated actions

## Goal

Create the gate core, `continual-gate`, with its first slice: from a command's segments of
words, recognise a commit, a push, a pull-request creation or a merge, and answer "ask" for
the actions its review-gate settings list in `ask`.

## Design

[Harness: the review gate](../../../docs/harness.md#the-review-gate) and the gate core's notes in
[Harness: design](../../../docs/harness.md#design). The crate is in
[Workspace: planned crates](../../../docs/workspace.md#planned-crates).

## Behaviours to test

- Commits, pushes, `gh pr create`, `gh pr merge` and a `gh api` call that writes to a
  repository's pulls are recognised, including through `git -C <dir>` and a full path to the
  program. A `gh api` call that only reads pulls is not.
- In a chain of segments, any gated segment makes the command gated.
- A recogniser in the settings' `commands` adds a tool, such as `glab mr create`.
- A gated action listed in `ask` is answered `ask`; one left out of `ask` is answered `pass`; the
  default `ask` lists all four.
- An unknown gated action in `ask` is rejected when the settings are built.
- Ordinary commands are answered `pass`, so the harness's own rules decide.
- The gate keeps no state: the same command under the same settings always gets the same
  answer, so a second commit asks like the first.

## Done when

- The rules are tested in the gate core with segment inputs and typed settings; no shell
  parsing and no configuration format is in it.
- `arch.toml` classifies the new core, which depends on no other workspace crate and carries
  a `clippy.toml` of its own.
