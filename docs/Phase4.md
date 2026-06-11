# Fase 4: Hardware Nativo y Síntesis de ADN Digital

## Objetivo
Despliegue en hardware especializado y computación bio-híbrida.

## Arquitectura

### Backend ADN
```
Qud::Zero  → "A" (Adenina)
Qud::One   → "C" (Citosina) 
Qud::Super → "G" (Guanina)
Qud::Error → "T" (Timina)
```

### Operaciones como Reacciones
| Operación | Reacción Bioquímica |
|-----------|-------------------|
| qadd | hybridization_add |
| qmul | enzymatic_mul |
| qnot | polymerase_not |
| collapse | restriction_collapse |

## Implementación

### JIT Backend
- **Archivo**: `src/jit.rs`
- **Cranelift**: Genera código objeto nativo
- **Feature flag**: `--features jit`
- **Función**: `emit_native()` compila QIR a código máquina

### Compilador Self-Hosted
- **Archivo**: `compiler.q`
- **Función**: Parsea, optimiza y emite código QUATRA
- **Target**: CPU (AVX2), FPGA (Verilog), ADN (secuencias)

### Verilog Backend
- **Archivo**: `src/verilog.rs`
- **Función**: `generate_verilog()` genera código FPGA synthesizable
- **4-state voltage mapping**:
  - Zero → `1'b0` (0V)
  - One → `1'b1` (1.0V)
  - Super → `2'b10` (0.5V intermedio)
  - Error → `2'b11` (1.5V estado error)
- **Módulo**: `fpga_intrinsics` con `estimate_resources()` y `pipeline_stages()`

### CLI Comandos Habilitados
```bash
quatra dna <file.q>      # Genera secuencia ADN (GenBank/FASTA)
quatra verilog <file.q>  # Genera código Verilog FPGA
quatra native <file.q>   # Compila con JIT (requiere --features jit)
```

## Verificación Bootstrap
```bash
# Compilar con todos los backends
cargo build --features jit

# Generar ADN de ejemplo
cargo run -- dna tests/samples/hello.q

# Generar Verilog de ejemplo
cargo run -- verilog tests/samples/hello.q
```

## Estado
- [x] ADN Backend implementado (`src/dna_assembly.rs` + `src/jit.rs::dna_backend`)
- [x] JIT Backend implementado (`src/jit.rs`)
- [x] Verilog Backend implementado (`src/verilog.rs`)
- [x] CLI integrado (`main.rs` comandos dna/verilog/native)
- [x] 33 tests pasando

## Métricas
- Tests: 33 (12 unit + 9 qud + 8 qir + 4 optimizer)
- Clippy: 0 warnings
- Features: `jit` con cranelift-object