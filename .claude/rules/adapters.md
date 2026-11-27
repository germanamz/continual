---
paths:
  - "crates/adapters/**"
  - "crates/continual/**"
---

# Adapter and composition-root rules

The full rules are in `CONVENTIONS.md`.

- Adapters only translate: parsing, serialisation, mapping, protocol details, error translation,
  resilience. Business decisions belong in a core (A1).
- A bridge joins two cores: it implements a driven port of one by calling the driving port of
  the other, maps between their types and errors, and depends on nothing but those two cores.
  It is named for both, the one that asks first (H2, H3).
- A driving adapter (CLI, harness hook) checks the input's shape, builds a core input, calls one
  core operation and maps the result and errors to its transport, such as a CLI exit code (A2,
  V1, E7).
- A driven adapter maps vendor types to core types; vendor types never cross the port (A4, B7).
  It translates technology errors that carry domain meaning into core errors, keeping the
  original as the source (E6).
- Timeouts, retries, authentication headers, pagination and rate limits live in the driven
  adapter (A3).
- Each adapter owns its DTOs and maps both ways; on-disk formats are DTOs, not core types with
  `serde` derives (M3, M4).
- A composition root builds each core after the driven adapters it needs, a bridge after the
  core it calls, and the driving adapters last (B5), and reads configuration to pass typed
  values to the cores (X4).
- The CLI prints only from an output module carrying
  `#![expect(clippy::print_stdout, reason = "…")]`.
- Long field-by-field mappings may exceed the length limit under a reasoned
  `#[expect(clippy::too_many_lines)]` (A6). Do not deduplicate similar adapters through a core
  (A7).
