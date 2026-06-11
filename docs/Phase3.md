# Fase 3: VM (Máquina Virtual Chimera)

## Objetivo
Ejecutar código QIR con soporte para operaciones cuaternarias y vectorizadas.

## Arquitectura ChimeraVM

### Registros
- `registers: HashMap<String, Qud>` - Almacenamiento de valores cuaternarios por nombre de registro
- `stack: QudStack` - Pila para operaciones y llamadas a funciones
- `functions: HashMap<String, QirFunction>` - Tabla de funciones cargadas

## Operaciones Implementadas

### Escalar (Escalar - Register-based)
| Instrucción | Operación |
|-------------|-----------|
| ConstQud | Asigna constante a registro |
| QAdd | (a + b) % 4 |
| QMul | (a * b) % 4, propagando Error |
| QNot | Rotación cíclica: Z→O→S→E→Z |
| Collapse | Super→One, Error→Zero |

### SIMD Vectoriales
| Instrucción | Operación |
|-------------|-----------|
| QAddVec | Adición elemento-wise en vectores |
| QMulVec | Multiplicación elemento-wise en vectores |
| QTranspose | Transposición de matrices (row-major ↔ col-major) |
| QDot | Producto punto acumulado |

### Control de Flujo
| Instrucción | Operación |
|-------------|-----------|
| Branch | Branch condicional (truthy = !Zero) |
| BranchIf | Branch explícito si condición == One |
| Jump | Salto incondicional |
| BranchTable | Dispatch por tabla (usado para pattern matching) |
| Phi | Unión de valores en join points |

### Memoria
| Instrucción | Operación |
|-------------|-----------|
| Store | Guarda valor en dirección de registro |
| Load | Carga valor desde dirección de registro |

## Optimizaciones

### Constant Folding
```
QMul("%t2", "a", "b")  // donde a=One, b=Super constantes
↓
ConstQud("%t2", Super)
```

### Dead Code Elimination
Elimina instrucciones Store cuyos valores nunca se cargan.

## Próximos Pasos
- [ ] Implementar memory heap para Store/Load
- [ ] Añadir tracing JIT para hot paths
- [ ] Implementar dispatch por perfilado de branches
- [ ] Generar código máquina (Fase 4)