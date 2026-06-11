---
description: Run QUATRA test suite
agent: compiler-dev
---
Run QUATRA tests and fix failures.

1. Run: `cargo test --all-targets`
2. For any failing tests, inspect the failure output.
3. Fix the root cause in `src/` or `tests/`.
4. Re-run tests until green.
5. Report: `X passed; Y failed`.
