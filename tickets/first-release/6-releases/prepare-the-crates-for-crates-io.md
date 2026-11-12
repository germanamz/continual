---
type: ticket
title: Prepare the crates for crates.io
blocks:
  - tickets/first-release/6-releases/publish-the-first-release
  - tickets/first-release/6-releases/propose-releases-with-release-please
parent: tickets/first-release/6-releases/story
points: 2
priority: medium
status: pending
---

# Prepare the crates for crates.io

## Goal

Make every product crate publishable to crates.io, with the metadata crates.io requires and
each adapter under its `continual-` name, checked on every pull request.

## Design

[Distribution: design](../../../docs/distribution.md#design).

## Behaviours to test

- `cargo publish --dry-run` succeeds for every product crate, in dependency order.
- Each product crate states its description, license, repository and readme.
- `xtask` cannot be published.

## Done when

- CI runs the dry run on pull requests.
