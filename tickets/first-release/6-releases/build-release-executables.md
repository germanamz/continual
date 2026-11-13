---
type: ticket
title: Build release executables
blocks: tickets/first-release/6-releases/publish-the-first-release
parent: tickets/first-release/6-releases/story
points: 3
priority: medium
status: pending
---

# Build release executables

## Goal

When release-please creates a release, build the prebuilt `continual` executables for every
platform the distribution design lists, attach them with their checksums, and publish the
product crates to crates.io.

## Design

[Distribution: behaviour](../../../docs/distribution.md#behaviour) and
[design](../../../docs/distribution.md#design).

## Behaviours to test

- A created release builds `continual` for Linux x86-64 and ARM64, macOS x86-64 and ARM64,
  and Windows x86-64, and attaches each with a SHA-256 checksum.
- Each executable runs `continual --version` on a machine with nothing else installed.
- The product crates are published in dependency order at the release's version.
- The workflow can be tried without publishing.

## Done when

- Third-party actions are pinned to full commit SHAs, like the other workflows.
