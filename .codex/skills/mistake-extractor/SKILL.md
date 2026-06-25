---
name: mistake-extractor
description: Convert Rust review feedback, debugging sessions, compiler errors, or user reflections into durable learning mistakes. Use when the user asks to extract mistakes, capture lessons, update mistakes.md, or identify mental-model gaps from a Rust exercise.
---

# Mistake Extractor

Use this skill to turn transient feedback into reusable learning material.

## Mistake Categories

- Correctness: panic, wrong output, missed edge case.
- Rust mental model: ownership, borrowing, lifetimes, traits, error propagation.
- Design: mixed responsibilities, awkward data flow, poor boundaries.
- API/CLI: confusing behavior, stdout/stderr mistakes, invalid inputs.
- Testing: missing tests, weak fixtures, untested failure path.
- Performance: unnecessary allocation, copying, buffering, or eager loading.

## Entry Format

```markdown
## Mistake: Short title

What happened:

Why it happened:

Better mental model:

How to recognize it next time:

Related code/review:
```

## Rules

- Preserve mistakes as learning artifacts, not as moral judgments.
- Prefer specific examples from the user's code.
- Separate actual bugs from idiomatic improvements.
- Do not invent mistakes that are not supported by the artifact.
