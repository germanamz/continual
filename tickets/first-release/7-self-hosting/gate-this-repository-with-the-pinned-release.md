---
type: ticket
title: Gate this repository with the pinned release
blocks:
  - tickets/first-release/7-self-hosting/replace-the-shell-hooks-with-the-claude-code-mod
  - tickets/first-release/7-self-hosting/retire-the-moved-xtask-checks
parent: tickets/first-release/7-self-hosting/story
points: 3
priority: medium
status: pending
---

# Gate this repository with the pinned release

## Goal

Make the pinned released `continual` the gate: CI installs the version `continual.toml` pins
and runs the pipeline and `continual guard` with it, and the git hooks call it, in place of
`cargo xtask`.

## Design

[Checks: self-hosting](../../../docs/checks.md#self-hosting) and
[Checks: CI](../../../docs/checks.md#ci).

## Behaviours to test

- CI installs exactly the pinned version and fails when the pin names one that does not
  exist.
- The gate jobs run `continual run` with the pinned release, and the guard runs as
  `continual guard --ci --labels <list>` in a job of its own.
- The dogfood job still runs the working-tree build.
- The git hooks run `continual event` with the pinned release.
- A change that weakens a built-in check passes the dogfood job and fails the gate.

## Done when

- The pinned release contains the pipeline, the built-in checks and the harness mod.
- The design documents describe the switch.
