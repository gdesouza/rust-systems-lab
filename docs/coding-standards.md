# Coding Standards

These standards apply when exercises or projects include runnable Rust code.

## Baseline

- Prefer clear control flow over clever compression.
- Keep allocation choices visible in systems-facing code.
- Return `Result` from composable operations instead of exiting deep inside
  helper functions.
- Separate formatting, I/O, parsing, and command-line concerns where practical.
- Test edge cases, especially partial input, invalid input, and error paths.

## Tooling

When a Cargo workspace exists, the default checks should be:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```
