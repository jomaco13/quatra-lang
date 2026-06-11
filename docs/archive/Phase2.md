# Fase 2: QIR (Quatra Intermediate Representation)

## Objetivo
Definir el lenguaje intermedio cuaternario para representar programas QUATRA antes de la ejecución en la VM.

## Arquitectura

### Formato Binario QIR
```
MAGIC:    0x49 0x52 ('Q' 'R')
VERSION:  0x02
FORMAT:   [func_count: u64 LE] [function...]
```

### Opcode Map
| Opcode  | Instrucción    | Operación                              |
|---------|----------------|----------------------------------------|
| 0x01    | ConstQud       | Carga constante cuaternaria            |
| 0x02    | QAdd           | Adición modular (a + b) % 4            |
| 0x03    | QMul           | Multiplicación cuaternaria             |
| 0x04    | QNot           | Rotación NOT (QNot: Z→O→S→E→Z)        |
| 0x05    | Collapse       | Colapso de Super/Error                 |
| 0x06    | Branch         | Branch condicional                     |
| 0x07    | BranchIf       | Branch explícito si/no                  |
| 0x08    | Jump           | Salto incondicional                    |
| 0x09    | Store          | Almacenar en memoria                   |
| 0x0A    | Load           | Cargar desde memoria                   |
| 0x0B    | Call           | Llamada a función                      |
| 0x0C    | Return         | Retorno de función                     |
| 0x0D    | Label          | Etiqueta de salto                      |
| 0x0E    | Phi            | Operación phi para SSA                 |
| 0x10    | QAddVec        | Adición SIMD vectorial                 |
| 0x11    | QMulVec        | Multiplicación SIMD vectorial          |
| 0x12    | QTranspose     | Transposición de matriz                  |
| 0x13    | QDot           | Producto punto cuaternario             |
| 0x14    | BranchTable    | Branch por tabla                       |

## Operaciones Cuaternarias en QIR

### QAdd: Adición Modular
```
QMul(dest, a, b)
result = (a.value + b.value) % 4
```
Tabla de verdad:
| a \ b | Z(0) | O(1) | S(2) | E(3) |
|-------|------|------|------|------|
| Z     | Z    | O    | S    | E    |
| O     | O    | S    | E    | Z    |
| S     | S    | E    | Z    | O    |
| E     | E    | Z    | O    | S    |

### QMul: Multiplicación Cuaternaria
```
QMul(dest, a, b)
result = (a.value * b.value) % 4
```
La multiplicación propaga `Error` inmunemente:
- Error × cualquier cosa = Error

### QNot: Rotación NOT
```
QNot(dest, src)
result = (src.value + 1) % 4  // Rotación cíclica
```

### Collapse: Colapso de Estados
```
Collapse(dest, src)
match src {
    Qud::Super => Qud::One,
    Qud::Error => Qud::Zero,
    _ => src
}
```

## SIMD Intrinsics

### QAddVec / QMulVec
Operaciones vectoriales con alineación de 32 bytes (AVX2/AVX-512):
```rust
pub const SIMD_ALIGNMENT: usize = 32;
```

La Fase 4 (hardware) implementará estas operaciones con verdadero SIMD.

### QTranspose
Transposición de matrices bidimensionales:
```
QTranspose(dest, src, rows, cols)
```
Reordena los elementos para operaciones matriciales eficientes.

### QDot
Producto punto para tensores:
```
QDot(dest, a, b, len)
result = Σ(a[i] * b[i]) para i en 0..len
```

## Traducción AST → QIR

El traductor (`qir_translator.rs`) convierte el AST a QIR:

1. **Expresiones literales** → `ConstQud`
2. **BinOp** → `QAdd`/`QMul` (QMax/QMin son placeholders)
3. **LetBind** → `Store` + binding en var_map
4. **FuncCall** → `Call`

## Próximos pasos
- [ ] Implementar ejecución real de SIMD intrinsics
- [ ] Añadir soporte para QMax/QMin con branches
- [ ] Optimizador QIR (Fase 3)