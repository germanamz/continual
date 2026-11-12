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

Build the prebuilt `continual` executables for every platform the distribution design lists,
with checksums, from a `v<version>` tag.

## Design

[Distribution: behaviour](../../../docs/distribution.md#behaviour) and
[design](../../../docs/distribution.md#design).

## Behaviours to test

- A release build produces `continual` for Linux x86-64 and ARM64, macOS x86-64 and ARM64,
  and Windows x86-64, each with a SHA-256 checksum.
- Each executable runs `continual --version` on a machine with nothing else installed.
- The workflow runs on a tag and can be tried without publishing.

## Done when

- The open question in docs/distribution.md is decided and removed.
- Third-party actions are pinned to full commit SHAs, like the other workflows.
