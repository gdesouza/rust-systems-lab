# Review: Exercise 01 Hex Dump

The first implementation was a strong starting point. It used small
abstractions and showed a natural instinct for decomposing the problem into
concepts such as chunks, formatting methods, and command-line parsing.

The next level of improvement is mostly about systems-programming discipline:
allocation control, streaming, separation of formatting from I/O, and
composable error handling.

## Review Focus

- Preserve the readable decomposition.
- Reduce unnecessary intermediate `String` allocation.
- Handle incomplete final rows with exact alignment.
- Stream input instead of reading the whole file.
- Keep formatting logic testable without requiring stdout or files.
- Return errors from helpers and decide how to present them at the CLI boundary.
