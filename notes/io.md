# I/O Notes

I/O-heavy Rust programs should make ownership, buffering, and error propagation
explicit.

## Hex Dump Lessons

- Prefer streaming input for files that may be large.
- Keep formatting logic separate from the writer that receives formatted text.
- Test incomplete final chunks because they usually expose alignment bugs.
- Treat stdout write failures as real errors.
