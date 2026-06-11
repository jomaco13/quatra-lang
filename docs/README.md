# QUATRA Documentation

## Architecture Overview

- **Phase 1**: Core - Lexer, parser, Qud types
- **Phase 2**: Q-IR - 19 opcodes, magic bytes 0x49 0x52 0x02
- **Phase 3**: Virtual Machine - Chimera VM with heap
- **Phase 4**: Hardware - JIT, Verilog FPGA, DNA synthesis

## Key Files

- `docs/Phase4.md` - Fase 4 implementation guide
- `docs/RESEARCH_REPORT.md` - Target audiences and applications
- `MAINTENANCE.md` - Project maintenance guide

## Language Basics

```quatra
-- 4 states: Zero(0), One(1), Super(2), Error(3)
-- Operators: <+> (qadd), <*> (qmul), ~~ (qnot), collapse
-- Functions: fun name(args) = expr
-- Let bindings: let x = val in expr
```

## Backend Commands

```bash
quatra compile file.q   # x86-64 ASM
quatra run file.q     # Interpreter
quatra vm file.q      # Chimera VM
quatra qir file.q   # Q-IR dump
quatra dna file.q     # DNA sequence (GenBank/FASTA)
quatra verilog file.q # FPGA Verilog
quatra native file.q  # JIT (requires --features jit)
```

## Examples

See `examples.q` and `tests/samples/*.q` for working QUATRA programs.