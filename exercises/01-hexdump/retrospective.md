# Retrospective: Exercise 01 Hex Dump

## What Rust Taught

- CLI argument parsing with `clap` integrates cleanly with structs. Even though
  this exercise did not require a CLI parser, seeing arguments represented as a
  typed struct was useful.
- `Option<T>` is the standard way to represent values that may be absent, such
  as optional `--width` and `--limit` arguments.
- Optional values need to be handled explicitly before they become ordinary
  values the program can use.

## What Surprised Me

- Unwrapping `Option<T>` values was less obvious than expected.
- The `chunks` iterator was a useful discovery for turning a byte slice into
  fixed-width rows.

## What I Would Redesign

- Rewrite the program in more idiomatic Rust.
- In particular, simplify optional argument handling, reduce manual control
  flow, and structure the formatter so the code reads more naturally.

## Go vs Rust Reflection

- Borrowing and lifetime syntax felt confusing compared with Go.
- Rust made absence explicit through `Option<T>`, while Go code would likely use
  zero values, pointers, or separate conditionals depending on the case.
- Rust's iterator tools, such as `chunks`, were expressive once discovered, but
  the surrounding type and ownership syntax made the first version feel heavier.
