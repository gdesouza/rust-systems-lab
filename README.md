# Rust Systems Lab

A long-running Rust systems programming lab, engineering notebook, and source
material for a future training course.

The goal is to preserve the full learning process: specifications, designs,
first attempts, reviews, mistakes, revisions, retrospectives, and course notes.
The final code matters, but the reasoning and review trail are what make the
repository useful as a portfolio and as teaching material.

## Repository structure

```text
.
├── README.md
├── ROADMAP.md
├── COURSE_NOTES.md
├── docs/                   # Working philosophy and durable engineering guidance
├── exercises/              # Focused exercises with review and reflection artifacts
├── projects/               # Larger systems projects
├── notes/                  # Topic-specific Rust and systems notes
└── templates/              # Reusable exercise/project document templates
```

## Current exercises

| Exercise | Project | Status | Focus |
| --- | --- | --- |
| 01 | Hex Dump | Capturing review artifacts | CLI design, byte formatting, error handling, streaming I/O |

## Working rhythm

For each exercise:

1. Start with an exercise brief and design notes.
2. Implement an initial version.
3. Review the code as a pull request, focusing on correctness, design, Rust
   idioms, and systems engineering practice.
4. Capture mistakes explicitly, including why they matter.
5. Revise the implementation.
6. Write a retrospective and a Go vs Rust reflection.
7. Promote durable lessons into `COURSE_NOTES.md`, `docs/`, or `notes/`.

## Code

This repository does not yet define a Cargo workspace. Add one when an exercise
or project includes runnable Rust code. Until then, the repository is
documentation-first and should not advertise build commands that do not exist.

## Course-building notes

The eventual course should be built from evidence in this repo: commits,
retrospectives, review feedback, improved versions, and recurring mistakes.
Prefer short notes after each session over long summaries at the end.
