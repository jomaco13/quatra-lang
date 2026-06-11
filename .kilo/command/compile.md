---
description: Compile a QUATRA source file to Q-IR, ASM, or native
agent: compiler-dev
subtask: true
---
Compile `$1` through the QUATRA pipeline.

1. Run: `cargo run -- compile "$1" --emit=asm`
2. If compilation fails, stop and report the error.
3. On success, show the generated ASM/Q-IR output.
