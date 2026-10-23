---
paths:
  - "**/*.rs"
  - "**/Cargo.toml"
  - "workers/**"
---

# Working in Rust

- Load the `rust-workers` skill. Rules (validation, formats, sessions) go in `crates/cms-core` with unit tests; workers stay thin.
- Nows is learning Rust from this code: prefer the plain version, and comment the Rust idea that carries the logic.
- Check with `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`.
