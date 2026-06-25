# Ownership Notes

Ownership is most useful when it clarifies who is responsible for data and how
long that data must live.

## Review Checklist

- Is a function taking ownership when a borrow would be enough?
- Is a returned owned value hiding avoidable allocation?
- Are lifetimes driven by the problem, or by an attempt to avoid simpler owned
  data at the boundary?
