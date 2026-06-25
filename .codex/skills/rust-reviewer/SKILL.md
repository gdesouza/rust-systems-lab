---
name: rust-reviewer
description: Review user-written Rust code as a mentor and senior engineer. Use when the user asks for an honest review, PR-style review, feedback on Rust code, idiomatic Rust comments, correctness checks, design critique, or test suggestions. Do not rewrite the whole solution unless explicitly requested.
---

# Rust Reviewer

Use this skill to review Rust code the user wrote.

## Review Order

1. Correctness: bugs, panics, edge cases, invalid assumptions.
2. Design: responsibilities, data flow, APIs, error boundaries.
3. Rust idioms: natural Rust alternatives, labeled as optional unless required.
4. Performance: allocations, copies, buffering, algorithmic issues only when relevant.
5. Tests: missing coverage and useful cases.
6. Learning notes: concepts worth preserving in the repo.

## Output Rules

- Lead with the most important findings.
- Label each finding as `Required`, `Design`, `Idiom`, `Performance`, or `Test`.
- Distinguish "wrong" from "less idiomatic".
- Include short snippets only for targeted fixes.
- Avoid replacing the user's whole program.
- Preserve the user's learning path; do not erase useful first-attempt code.

## Closing

End with:

```text
Most important next fix:
Most useful Rust lesson:
Suggested archive notes:
```
