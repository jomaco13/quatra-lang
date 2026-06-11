# Fase 3: VM (Máquina Virtual Chimera)

## Objetivo
Ejecutar código QIR con soporte para operaciones cuaternarias y vectorizadas usando software.

## Arquitectura ChimeraVM

### Registros
- `registers: HashMap<String, Qud>` - Almacenamiento de valores cuaternarios por nombre de registro
- `stack: QudStack` - Pila para operaciones y llamadas a funciones
- `functions: HashMap<String, QirFunction>` - Tabla de funciones cargadas

## Operaciones Implementadas

### Escalar (Register-based)
| Instrucción | Operación |
|-------------|-----------|
| ConstQud | Asigna constante a registro |
| QAdd | (a + b) % 4 |
| QMul | (a * b) % 4, propagando Error |
| QNot | Rotación cíclica: Z→O→S→E→Z |
| Collapse | Super→One, Error→Zero |

### SIMD Vectoriales (Software Simulation)
**Nota: Sin hardware especializado, usamos paralelismo via iteradores.**

| Instrucción | Operación |
|-------------|-----------|
| QAddVec | Adición elemento-wise con iteradores paralelos |
| QMulVec | Multiplicación elemento-wise |
| QTranspose | Transposición de matrices (row-major ↔ col-major) |
| QDot | Producto punto acumulado con fold |

### Control de Flujo
| Instrucción | Operación |
|-------------|-----------|
| Branch | Branch condicional (truthy = !Zero) |
| BranchIf | Branch explícito si condición == One |
| Jump | Salto incondicional |
| BranchTable | Dispatch por tabla (pattern matching) |
| Phi | Unión de valores en join points |

## Optimizaciones

### Constant Folding
```
QMul("%t2", "a", "b")  // donde a=One, b=Super constantes
↓
ConstQud("%t2", Super)
```

### Dead Code Elimination
Elimina instrucciones Store cuyos valores nunca se cargan.

## Simulación SIMD eficiente (sin hardware especializado)

Para máquinas sin AVX-512, usamos:
1. **Iteradores paralelos**: `rayon` para operaciones vectoriales
2. **Prefetch manual**: Acceso secuencial a registros
3. **Batch processing**: Operaciones en lotes de 4 valores

```rust
// Ejemplo: QAddVec sin SIMD hardware
fn simd_add(&mut self, dest: &str, a: &str, b: &str, len: usize) {
    // Los valores se almacenan como: vec_0, vec_1, vec_2...
    for i in 0..len {
        let a_val = self.registers.get(&format!("{}_{}", a, i)).copied().unwrap_or(Qud::Zero);
        let b_val = self.registers.get(&format!("{}_{}", b, i)).copied().unwrap_or(Qud::Zero);
        self.registers.insert(format!("{}_{}", dest, i), a_val + b_val);
    }
}
```

## Próximos pasos
- [ ] Integración con rayon para paralelismo real
- [ ] Profiling de hot paths
- [ ] JIT con cranelift (generación de código nativo x86-64/ARM64)
- [ ] Memory heap para Store/Load dinámico