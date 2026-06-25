---
name: exercise-archivist
description: Archive Rust learning exercises into this repository's course-notebook structure. Use when the user asks to populate an exercise directory, preserve submitted code, extract review artifacts, update starter or solution folders, or turn chat history into exercise files.
---

# Exercise Archivist

Use this skill to preserve real learning artifacts from chats, code submissions,
reviews, and retrospectives.

## Repository Assumptions

Prefer this structure when present:

```text
exercises/NN-name/
├── README.md
├── starter/
├── solution/
├── review.md
├── mistakes.md
├── retrospective.md
└── extensions.md
```

## Workflow

1. Locate the relevant source artifact: chat export, pasted code, diff, or existing file.
2. Preserve the user's original attempt under `starter/`.
3. Put revised/reference code under `solution/` only when explicitly available or requested.
4. Extract review feedback into `review.md`.
5. Extract durable mistakes into `mistakes.md`.
6. Prepare retrospective prompts but do not invent the user's personal reflection.
7. Add follow-up ideas to `extensions.md`.
8. Keep `.codex/` untracked unless the user explicitly asks to commit it.

## Guardrails

- Do not polish away evidence of the user's original attempt.
- Do not fabricate missing code.
- Do not add CI, workspace files, or scaffolding unrelated to the exercise.
- If source code is in chat exports, cite the file and line before copying it.
