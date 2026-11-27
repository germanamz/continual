---
paths:
  - "crates/libraries/**"
---

# Library crate rules

The full rules are H6 to H8 in `CONVENTIONS.md`, and the design is in `docs/workspace.md`.

- A library crate holds data that two or more cores mean the same way: plain data records
  with public fields and no behaviour, and value objects whose invariant is about the value
  alone (Q13). It holds nothing that only one core owns: no rule, error or concept of one
  core (H6).
- No trait and no use case: ports stay in the cores (P7). A library crate is neither a port
  nor an adapter, only a dependency of the crates that use it (H6).
- It keeps a core's limits: no technology dependency, no I/O, clock, environment, process or
  randomness, no `serde`, an empty allow-list of its own in `arch.toml` and a `clippy.toml`
  of its own. It depends on no workspace crate, another library crate included (H7).
- Every change here is a guarded change. Report it to the contributor when the change is
  ready, and never add the approval label yourself (H8, R2).
- A change is backwards compatible: add, and never remove, rename or alter what a crate
  already uses. A new field on a record or a new variant of an enum is not backwards
  compatible, so it arrives as a new item beside the old one, and the old one goes in a
  planned cleanup ticket once nothing uses it (H8).
- Adding what no ticket names is a stop-and-report choice.
