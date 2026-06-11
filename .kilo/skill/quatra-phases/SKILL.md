/*
Module:       .kilo/skill/quatra-phases/SKILL.md
Description:  Guia profesional para desarrollar QUATRA en fases con Kilo.
Trigger:      quatra phases, quatra roadmap, quatra phase 1, quatra phase 2, quatra phase 3, quatra phase 4, quatra plan, quatra implementation
*/

# QUATRA Professional Development Skill

Use this skill when implementing or extending the QUATRA language in phases.

## Active config
- Project kilo config: `.kilo/kilo.json`
- Kilo root instructions: `CLAUDE.md`, `CONTEXT.md`

## Phase roadmap
- Phase 1: Core (Qud, lexer, parser, AST, interpreter)
- Phase 2: Q-IR + codegen
- Phase 3: Chimera VM + Nucleo + SIMD
- Phase 4: FPGA + DNA backends

## Workflow
1. Pick one phase.
2. Use `compiler-dev` for implementation.
3. Use `qa-reviewer` before merging changes.
4. Use `/test` until green.
5. Use `/lint` before review.

## Commands
- `/compile <file.q>`: compile to Q-IR/ASM.
- `/run <file.q>`: execute via interpreter/VM.
- `/test`: full cargo test suite.
- `/fmt`: rustfmt.
- `/lint`: clippy strict.
