# QUATRA - Ruta a Self-Hosting

## Arquitectura de 4 Fases

### Fase 1: Core (VERDE)
- Tipo `Qud` (cuaternario): Zero, One, Super, Error
- Operaciones: Add, Mul, Not, Max, Min, Collapse
- Lexer y parser básicos

### Fase 2: QIR (VERDE) 
- Formato binario con magic bytes `0x49 0x52 0x02`
- 19 opcodes con serialización/deserialización
- Instrucciones SIMD: QAddVec, QMulVec, QTranspose, QDot

### Fase 3: VM (VERDE)
- ChimeraVM register-based + stack
- Ejecución de instrucciones QIR
- Optimizer con constant folding

### Fase 4: Native Code Generation (PENDIENTE)
- **Cranelift JIT**: generación de código x86-64/ARM64
- **Memory heap**: para Store/Load dinámico
- **Profiling**: hot path detection para optimización

## Bootstrap a Self-Hosting

### Stage 0 (Actual)
Compilador escrito en Rust → compila programas QUATRA

### Stage 1 (Fase 4a)
```
Rust Compiler → compiler.q → compiler_stage1 (nativo)
```
El compilador Rust compila `compiler.q`, generando el primer binario QUATRA

### Stage 2 (Fase 4b) 
```
compiler_stage1 → compiler_stage2.q → compiler_stage2
```
El binario QUATRA compila versiones mejoradas de sí mismo

### Stage 3 (Validación)
```
compiler_stage2 → compiler_stage2.q → compiler_stage2_verified
```
Si `compiler_stage2 == compiler_stage2_verified`, QUATRA está autoalojado

## Benchmark Objetivo Self-Hosting

1. El compilador QUATRA debe poder:
   - Parsear su propio código fuente (`compiler.q`)
   - Generar código máquina para sus propias instrucciones
   - Ejecutar de forma iterativa el proceso de bootstrap

2. Criterios de éxito:
   - `diff compiler_stage2 compiler_stage2_verified == 0`
   - Compilación de QUATRA a QUATRA sin Rust intermedio

## Extensión de archivo
- Fuente: `.q`
- Bytecode QIR: `.qir`
- Binary: platform-specific (`.exe`, sin extensión)