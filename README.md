# Rust Systems Lab

A hands-on lab repository for building small systems-programming projects in Rust.

The goal is to grow this repo into both a learning portfolio and the raw material for a future training course. Each lab should capture not only the final code, but also the design choices, trade-offs, mistakes, and review notes that made the exercise useful.

## Repository structure

```text
.
├── crates/                 # Rust binaries and libraries built during the labs
├── labs/                   # Exercise briefs, requirements, and review prompts
├── docs/                   # Learning trajectory, retrospectives, and course notes
└── .github/workflows/      # CI checks
```

## Current labs

| Lab | Project | Focus |
| --- | --- | --- |
| 01 | Hex Viewer | CLI design, byte formatting, error handling, streaming I/O |

## Working rhythm

For each exercise:

1. Start from the lab brief in `labs/`.
2. Implement the project under `crates/`.
3. Run formatting, linting, and tests.
4. Capture a retrospective in `docs/learning-log.md`.
5. Promote durable lessons into `docs/course-outline.md`.

## Commands

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Course-building notes

The eventual course should be built from evidence in this repo: commits, retrospectives, review feedback, improved versions, and recurring mistakes. Prefer short notes after each session over long summaries at the end.
