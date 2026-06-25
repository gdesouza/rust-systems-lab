# Exercise 01: Hex Dump

Build a command-line program that prints file bytes in a readable hexadecimal
view.

This exercise is being captured after the initial implementation and review.
The purpose of this directory is to preserve the assignment, review notes,
mistakes, retrospective, and future extension ideas.

## Learning Goals

- Practice CLI structure in Rust.
- Format binary data predictably.
- Handle incomplete final rows.
- Separate formatting from output.
- Prefer streaming I/O for large inputs.
- Propagate errors with `Result` where composition is useful.

## Expected Behavior

A typical hex dump view includes:

- an offset column
- hexadecimal byte values
- aligned spacing for incomplete final rows
- an ASCII preview column
- useful errors for missing or unreadable inputs

## Artifacts

- `review.md` records code-review feedback.
- `mistakes.md` preserves mistakes as teaching material.
- `retrospective.md` captures what changed after review.
- `extensions.md` lists follow-up work.
- `starter/` can hold the initial attempt when available.
- `solution/` can hold a revised or reference implementation when available.
