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

Merge the first release pull request, so the workflows tag, build and publish Continual's first
release, and `cargo install continual` and the downloaded executable both work.

## Design

[Distribution](../../../docs/distribution.md): every section.

## Behaviours to test

- `cargo install continual --locked --version <version>` installs the release.
- The GitHub release carries every executable and checksum the release build produces.
- A downloaded executable reports the tagged version.

## Done when

- The contributor has merged the first release pull request; releasing is theirs to do.
- The design documents describe the change, and the planned markers of
  [Publish Continual releases](story.md) are gone.
