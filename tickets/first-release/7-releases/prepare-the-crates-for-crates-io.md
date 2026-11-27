---
type: ticket
title: Prepare the crates for crates.io
blocks:
  - tickets/first-release/7-releases/publish-the-first-release
  - tickets/first-release/7-releases/propose-releases-with-release-please
parent: tickets/first-release/7-releases/story
points: 2
priority: medium
status: pending
---

# Prepare the crates for crates.io

## Goal

Make every product crate publishable to crates.io, with the metadata crates.io requires and
each core and adapter under its `continual-` name, checked on every pull request.

## Design

[Distribution: design](../../../docs/distribution.md#design).

## Carried over

- Library crates are product crates and go first in the dependency order.
  `continual-vocabulary` is among them. It sets `publish = false`, as the cores do, and the
  engine depends on it by path alone, where crates.io asks for a version beside the path.

## Behaviours to test

- `cargo publish --dry-run` succeeds for every product crate, in dependency order.
- Each product crate states its description, license, repository and readme.
- `xtask` cannot be published.

## Done when

- CI runs the dry run on pull requests.
