# Rust Patterns

This document collects reusable Rust patterns noticed across exercises.

## Candidate Patterns

- Formatter-oriented APIs can avoid unnecessary intermediate `String`
  allocation.
- Iterator pipelines are useful, but allocation inside each iteration should be
  intentional.
- Small pure functions make CLI behavior easier to test.
- `Result`-based composition keeps error handling visible at the boundary where
  policy decisions are made.
