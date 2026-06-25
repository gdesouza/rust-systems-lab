# Review: Exercise 01 Hex Dump

The first implementation has a good shape for a first Rust systems exercise:
there is a clear `Args` type, a small `Chunk` abstraction, stdout is locked, and
broken pipes are handled intentionally. The main work now is to make the code
compile cleanly, simplify the control flow, and make formatting precise enough
to test.

## Findings

### Required: the archived starter does not compile

`cargo check` fails in `starter/src/main.rs`.

- `get_limit` has two tail expressions next to each other:
  `limit` followed by `arg_limit.unwrap_or(max_limit).min(max_limit)`.
- `get_width` has `return buffer_size` without a semicolon, followed by another
  expression.
- `buffer.data` is reassigned even though `buffer` is not mutable.

The important lesson is not just "add semicolons". The code is mixing two Rust
return styles in the same function. Pick one expression-oriented version:

```rust
fn get_limit(arg_limit: Option<usize>, max_limit: usize) -> usize {
    arg_limit.unwrap_or(max_limit).min(max_limit)
}

fn get_width(arg_width: Option<usize>) -> usize {
    arg_width.unwrap_or(16)
}
```

For the chunk trimming, prefer computing the limited slice before chunking
instead of mutating a wrapper after the fact:

```rust
for (row, chunk) in data[..limit].chunks(buffer_size).enumerate() {
    let offset = row * buffer_size;
    // ...
}
```

### Required: incomplete final rows are not aligned

`to_hex` only prints bytes that exist. For a final row shorter than `width`, the
ASCII column shifts left. A hex dump should keep the ASCII preview in a stable
column, so the renderer needs to pad missing byte slots.

Example shape for `width = 4` and two final bytes:

```text
00000004:   61 62       | ab |
```

This is a formatting correctness issue because the output becomes harder to
scan and compare.

### Design: formatting and output are still coupled

`main` owns argument parsing, file reading, limiting, row iteration, formatting,
stdout locking, and write-error policy. That is manageable here, but it makes
tests awkward because the only natural test path is running the whole binary.

A useful next split is:

- one function that formats a single row into any `Write`
- one function that dumps bytes into any `Write`
- `main` as the CLI boundary that opens files and decides how to report errors

That keeps the exercise small while making output behavior testable with a
`Vec<u8>`.

### Design: reading the whole file is acceptable for the first attempt, but not
the systems version

`fs::read` is simple and correct for small files. For a hex dump tool, the next
step should use `File` plus buffered reads so large inputs do not require memory
proportional to file size.

The current `--limit` handling also becomes cleaner with streaming: stop after
emitting `limit` bytes instead of reading the entire file and slicing later.

### Idiom: avoid `format!` inside the byte iterator when writing output

This code:

```rust
.map(|byte| format!("{:02x} ", byte))
.collect()
```

allocates a tiny `String` per byte and then collects those into another
`String`. It is valid Rust, but expensive for the job. When the destination is
already a writer, prefer `write!(out, "{byte:02x} ")` inside a loop.

This is a good Rust-specific lesson: iterator chains are not automatically
allocation-free. The allocation behavior depends on what the closure returns.

### Idiom: the `Chunk` wrapper is reasonable, but its responsibility needs to be
clear

`Chunk<'a> { data: &'a [u8] }` is a good first use of borrowing. However, if the
wrapper only converts to owned `String`s, it is mostly a naming layer. It becomes
more useful if it either:

- renders itself into a formatter/writer, or
- remains a tiny view type used by formatting functions.

Do not mutate the wrapper to shorten data. Compute the correct slice first and
then wrap it.

### Performance: stdout locking is good; allocation strategy is the next issue

Locking stdout once is the right instinct. The biggest remaining cost is not
stdout itself, but repeated allocation in `to_hex` and `to_ascii`, plus reading
the full file into memory.

For this exercise, fix allocation in formatting before moving to streaming I/O.
That gives you a smaller, testable improvement before changing the data flow.

### Test: add output tests before refactoring

Useful test cases:

- empty input emits no rows
- exactly one full row
- final row shorter than width keeps ASCII alignment
- `--limit` stops in the middle of a row
- non-printable bytes render as `.`
- space renders as a visible ASCII space, not `.`
- `--width 0` is rejected

The most valuable first test is the incomplete final row, because it catches
both limiting and padding mistakes.

## Suggested Next Patch

1. Make the starter compile without changing behavior more than necessary.
2. Replace the manual `get_limit` and `get_width` matches with
   `unwrap_or(...).min(...)` style expressions.
3. Slice to `data[..limit]` before calling `.chunks(buffer_size)`.
4. Add a row-formatting test for a partial final row.
5. Only then change row formatting to write into a `Write` without intermediate
   `String`s.

Most important next fix: make `starter/src/main.rs` compile by removing the
duplicate return expressions and avoiding mutation of an immutable `Chunk`.

Most useful Rust lesson: Rust functions are expressions by default, so mixing
assignment-heavy style with tail expressions can create confusing and invalid
control flow.

Suggested archive notes: Preserve the allocation-vs-iterator lesson, the
borrowed `Chunk<'a>` idea, and the partial-row alignment bug as durable learning
points.
