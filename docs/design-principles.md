# Design Principles

Every exercise should begin with a small design pass. The design does not need
to be long, but it should answer enough questions to make the implementation
intentional.

## Questions To Answer

- What inputs are accepted?
- What outputs are promised?
- What errors are expected?
- What data should stream instead of being loaded eagerly?
- Where are allocations acceptable?
- Which parts are pure logic and which parts perform I/O?
- What behavior deserves tests before refactoring?
