---
type: ticket
title: Publish the first release
parent: tickets/first-release/6-releases/story
points: 1
priority: medium
status: pending
---

# Publish the first release

## Goal

Publish the first Continual release to crates.io and GitHub, so `cargo install continual` and
the downloaded executable both work and a repository can pin it.

## Design

[Distribution](../../../docs/distribution.md): every section.

## Behaviours to test

- `cargo install continual --locked --version <version>` installs the release.
- The GitHub release carries every executable and checksum the release build produces.
- A downloaded executable reports the tagged version.

## Done when

- The contributor has tagged and published the release; publishing is theirs to do.
- The design documents describe the change, and the planned markers of
  [Publish Continual releases](story.md) are gone.
