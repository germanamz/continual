# Distribution

> Planned: [Publish Continual releases](../tickets/first-release/6-releases/story.md)

How Continual reaches the people and agents using it: a single executable, installed with
cargo, built from source or downloaded, and the releases that repositories pin in their
`continual.toml` ([Pipeline](pipeline.md#continualtoml)).

## Concepts

- **Release pull request**: the pull request release-please keeps open on `main`, holding the
  next version number and the changelog drawn from the Conventional Commits merged since the
  last release.

## Behaviour

- `cargo install continual --locked` installs the latest release from crates.io, and
  `cargo install continual --locked --version <version>` a pinned one.
- `cargo build --release --package continual` builds it from a clone of the repository.
- Each release on GitHub attaches a prebuilt `continual` executable for Linux (x86-64 and
  ARM64), macOS (x86-64 and ARM64) and Windows (x86-64), each with a SHA-256 checksum.
- Every form is the same self-contained executable. Every language adapter is compiled in;
  there are no plugins to install and no runtime to provide.
- When a repository pins a version that the installed binary is not, Continual prints the
  `cargo install` command for the pinned version.
- A release happens when a contributor merges the release pull request. Nothing is tagged,
  built or published by hand.

## Design

- release-please runs on every push to `main`. From the Conventional Commits since the last
  release it opens or updates the release pull request, which bumps the version every product
  crate inherits from `[workspace.package]` and adds the changelog entry. The commit types
  decide the bump, as the release-please configuration in the repository sets it.
- Merging the release pull request makes release-please tag `v<version>` and create the GitHub
  release. A release workflow, triggered by that release, builds the executables on each
  platform, attaches them with their checksums, and publishes the product crates to
  crates.io in dependency order.
- The product crates (`continual-core`, the adapters and `continual`) share one version, set
  in `[workspace.package]`, and are published together. Adapter crates are published under
  `continual-` names, such as `continual-git`, while their directories keep the short names.
  `xtask` is never published.
- Pinning this repository to the new release is a separate pull request
  ([Checks: self-hosting](checks.md#self-hosting)).

## Decisions

- **Language adapters are compiled in.** One executable is all a repository needs, on every
  platform, and the binary a version pins is exactly the code that runs. Rejected: plugins
  loaded at run time, which bring ABI concerns, a second distribution channel, and code a pin
  does not cover.
- **Three ways to install.** `cargo install` suits Rust contributors, the prebuilt executable
  suits every other language and CI, and building from source suits contributors to
  Continual.
- **Releases go through a release pull request, with release-please.** A release is reviewed
  and merged like any change, its version and changelog come from the Conventional Commits the
  repository already enforces, and tagging, building and publishing follow from the merge
  without a hand-run step. Rejected: tagging releases by hand, which skips review and leaves
  the version to memory; and generating the whole release with cargo-dist, which builds
  executables but neither chooses the version nor writes the changelog.
