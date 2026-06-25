# Mistakes: Exercise 01 Hex Dump

These are not failures. They are the concrete review findings that make the
exercise useful as course material.

## Findings To Preserve

1. Returning owned `String`s everywhere instead of writing directly to a
   formatter or writer.
2. Allocating with `format!` inside iterator pipelines.
3. Missing width and alignment handling for incomplete final rows.
4. Reading the whole file instead of streaming.
5. Mixing formatting logic with output logic.
6. Limited error propagation, favoring explicit exits over `Result`-based
   composition.
7. Missing opportunities to use iterators without intermediate allocations.

## Why These Matter

The program can be correct for small inputs while still hiding costs that matter
in systems code. Hex dumping is a useful exercise because the output format is
simple enough to understand, but strict enough to expose ownership, allocation,
buffering, and formatting decisions.
