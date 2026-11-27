---
type: ticket
title: Propose releases with release-please
blocks: tickets/first-release/7-releases/build-release-executables
parent: tickets/first-release/7-releases/story
points: 2
priority: medium
status: pending
---

# Propose releases with release-please

## Goal

Make every release a pull request: release-please keeps a release pull request open on `main`
with the next version and the changelog, and merging it tags the release and creates it on
GitHub.

## Design

[Distribution: design](../../../docs/distribution.md#design) and the release-please decision in
[decisions](../../../docs/distribution.md#decisions).

## Behaviours to test

- A push to `main` with a `feat` or `fix` commit opens or updates the release pull request.
- The release pull request bumps the version every product crate inherits from
  `[workspace.package]`, and `Cargo.lock` with it, and adds the changelog entry.
- Merging it tags `v<version>` and creates the GitHub release.
- A push with only `docs`, `test` or `chore` commits proposes no release.
- The release pull request runs the same checks as any pull request, and the release created
  by merging it starts the release workflow, because release-please runs with its own token.

## Done when

- The contributor has created the token and stored it as a repository secret; creating it is
  theirs to do.
- The workflow and the release-please configuration are in `.github/`, with third-party
  actions pinned to full commit SHAs.
