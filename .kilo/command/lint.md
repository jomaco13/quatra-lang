---
description: Lint QUATRA code with clippy
agent: compiler-dev
---
Run Clippy lints.

1. Run: `cargo clippy --all-targets -- -D warnings`
2. Fix any lints that cause build failure.
3. Re-run until clean.
