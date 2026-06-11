# QUATRA Project Maintenance Guide

## Overview
Maintener un lenguaje de programación funcional cuaternario (base-4) con múltiples backends.

---

## Development Workflow

### Branch Strategy
```bash
main        # Production-ready code
develop     # Active development
feature/*   # New features (dna-backend, verilog-improvements)
hotfix/*    # Bug fixes
release/*   # Version tags
```

### Daily Development Commands
```bash
# Run tests
cargo test

# Format code
cargo fmt

# Lint check
cargo clippy --all-targets

# Build all features
cargo build --features jit --release

# Check specific backend
cargo run -- dna file.q
cargo run -- verilog file.q
```

---

## Testing Protocol

### Test Categories
1. **Unit Tests** (`src/*_test.rs`) - Individual functions
2. **Integration Tests** (`tests/*.rs`) - QIR compilation, optimizer
3. **Sample Tests** (`tests/samples/*.q`) - Language features

### Test Execution
```bash
# All tests
cargo test

# Specific module
cargo test verilog::tests

# With output
cargo test -- --nocapture
```

### Coverage Targets
- Fase 1 (lexer/parser): 12 tests
- Fase 2 (QIR): 8 tests
- Fase 3 (VM): 9 tests
- Fase 4 (backends): 4 tests

---

## Code Standards

### Rust Conventions
- **Formatter**: `cargo fmt` (rustfmt)
- **Linter**: `cargo clippy` (zero warnings)
- **Doc comments**: `///` for public APIs

### QUATRA Language Features
- Operadores: `<+> <*> ~~`
- Functions: `fun name(args) = expr`
- Let bindings: `let x = val in expr`

### New OpCode Addition
Edit `src/qir.rs`:
1. Add to `Opcode` enum
2. Add to `from_u8`/`to_u8`
3. Add serialization in `to_bytes`
4. Add deserialization in `from_bytes`
5. Add test in `tests/qir_test.rs`

---

## Release Process

```bash
# 1. Update version in Cargo.toml
# 2. Run full test suite
cargo test && cargo clippy --all-targets

# 3. Create release
git tag -a v0.X.0 -m "Release v0.X.0"
git push origin --tags
gh release create v0.X.0 --notes "Release notes"

# 4. Publish crates.io (optional)
cargo publish
```

---

## Issue Management

### Labels
- `phase/1-core` - Lexer/parser
- `phase/2-qir` - Intermediate representation
- `phase/3-vm` - Virtual machine
- `phase/4-hardware` - FPGA/DNA/JIT
- `bug` - Regression
- `enhancement` - New features

### Priority System
- **Critical**: Tests failing, build broken
- **High**: Missing opcodes, backend errors
- **Medium**: Syntax extensions, stdlib
- **Low**: Documentation, examples

---

## CI/CD (Si configurar)

```yaml
# .github/workflows/ci.yml
name: QUATRA CI
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --all-features
      - run: cargo clippy --all-targets --all-features
```

---

## Architecture Maintenance

### Adding New Backend
1. Create `src/backend_name.rs`
2. Add module in `src/lib.rs`
3. Add CLI command in `src/main.rs`
4. Add tests in `src/backend_name.rs`

### Memory Management
- Use Rust ownership (no GC)
- SIMD_ALIGNMENT = 32 bytes
- Heap management in `vm/interpreter.rs`

### Unsafe Code Policy
- Only in `vm/intrinsics.rs` for performance
- Document every `unsafe` block
- Test thoroughly

---

## Community Engagement

### For Researchers
- Academic papers: arXiv links
- Conference submissions: PLDI, OOPSLA

### For Hardware Engineers
- FPGA examples in `tests/samples/`
- Verilog generation examples

### For Developers
- Language tour in `examples/`
- Quickstart in README.md

---

## Backup & Recovery

```bash
# Monthly backup
git bundle create quatra-backup.bundle --all

# Recovery
git clone quatra-backup.bundle
```

---

## Metrics Dashboard (Manual)

| Metric | Target | Current |
|--------|--------|---------|
| Tests passing | 100% | 33/33 |
| Clippy warnings | 0 | 0 |
| QIR opcodes | 19 | 19 |
| Backends | 5 | 5 (ASM, VM, JIT, Verilog, DNA) |

---

## Contact
Maintained by: jomaco13
Repository: https://github.com/jomaco13/quatra-lang