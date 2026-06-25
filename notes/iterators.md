# Iterator Notes

Iterators are a strength of Rust, but they do not automatically make code
allocation-free or easier to read.

## Review Checklist

- Does the iterator pipeline allocate on every item?
- Would a simple loop make ownership or error handling clearer?
- Are intermediate collections necessary?
- Can the operation write into an existing formatter or buffer?
