---
type: ticket
title: Retire the moved xtask checks
parent: tickets/first-release/7-self-hosting/story
points: 3
priority: low
status: pending
---

# Retire the moved xtask checks

## Goal

Remove from `xtask` every check the pinned release now runs (comments, arch, guard,
attribution and the `ci` pipeline), keeping only what Continual does not cover, so each check
has one implementation.

## Design

[Checks: self-hosting](../../../docs/checks.md#self-hosting).

## Behaviours to test

- Each removed check has a built-in check that covers its cases, and its tests moved there.
- What stays in `xtask`, such as coverage thresholds and mutation testing, is run from
  `continual.toml` as tool steps.
- `arch.toml`, `lefthook.yml` and the CI workflows no longer name the removed checks.

## Done when

- docs/checks.md describes the checks as Continual runs them, and the planned markers of
  [Check Continual with a pinned release](story.md) are gone.
