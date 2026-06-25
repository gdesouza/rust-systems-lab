# Roadmap

This roadmap is the source of truth for journey state. Update the `Status` and
`Notes` columns whenever an exercise moves forward.

Status meanings:

- `Planned`: not started.
- `Preparing`: lesson or design prep is underway.
- `Implementing`: the user is writing the first attempt.
- `Reviewing`: submitted code is being reviewed.
- `Archiving`: code, review, mistakes, and retrospective artifacts are being
  copied into the repo.
- `Complete`: artifacts are captured and the exercise can be used as course
  evidence.

| No. | Exercise or Project | Status | Notes |
| --- | --- | --- | --- |
| 01 | Hex Dump | Archiving | Source and review are in `.codex/chat-6a3c8c8af49881918136e633beed8f89-03.md`; copy the first attempt into `starter/` and reconcile `review.md` and `mistakes.md`. |
| 02 | Strings | Planned | Next exercise after Hex Dump is archived; focus on byte streams, UTF-8, buffering, state machines, and tests. |
| 03 | Process Monitor | Planned | Inspect processes and practice OS-facing Rust. |
| 04 | Read-only Linux Explorer | Planned | Navigate Linux system information safely. |
| 05 | Robot Diagnostic Toolkit | Planned | Build practical CLI diagnostics for robotics-style systems. |
| 06 | Binary Log Decoder | Planned | Parse structured binary data and handle malformed input. |
| 07 | Secure Log Collection Agent | Planned | Combine file I/O, networking, security, and operational concerns. |
| 08 | Teleport Restricted CLI | Planned | Build toward a secure, constrained operational CLI. |
| 09 | Mini Container Runtime | Planned | Learn process isolation and Linux primitives. |
| 10 | eBPF Performance Inspector | Planned | Explore low-level observability. |
| 11 | Linux Image Builder | Planned | Assemble a larger systems tool with clear architecture. |

## Operating Rules

1. Design before implementation.
2. Tests are part of the project.
3. Review before refactoring.
4. Preserve mistakes as teaching material.
5. End each exercise with a retrospective and a Go vs Rust reflection.
