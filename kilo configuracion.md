Para configurar **Kilo Code** y codificar todo el plan de **QUATRA**, sigue esta configuración basada en las mejores prácticas de 2026:

### 1. **Configuración Inicial del Proyecto**

Crea un archivo `kilo.jsonc` en la raíz del proyecto para definir agentes personalizados:

```
\{  
  "$schema": "https://app.kilo.ai/config.json",  
  "agent": \{  
    "compiler-dev": \{  
      "description": "Desarrolla el compilador QUATRA en Rust",  
      "mode": "primary",  
      "model": "anthropic/claude-3-5-sonnet-20241022",  
      "permission": \{  
        "edit": "allow",  
        "bash": "allow",  
        "read": "allow"  
      \}  
    \},  
    "qa-reviewer": \{  
      "description": "Revisa código para calidad y seguridad",  
      "mode": "subagent",  
      "model": "openai/gpt-4o-2024-08-06",  
      "permission": \{  
        "edit": "deny",  
        "bash": "deny"  
      \}  
    \}  
  \},  
  "memory-bank": true  
\}
```

### 2. **Estrategia de Prompt Agéntico (RCTFE)**

Usa el agente `compiler-dev` con prompts estructurados:

- **Rol:** "Eres un ingeniero principal de compiladores especializado en lenguajes de bajo nivel."

- **Contexto:** "Estás desarrollando QUATRA, un lenguaje funcional cuaternario. El código base está en Rust. La meta es el autoalojamiento."

- **Tarea:** "Implementa la función `qadd\_simd` en `src/vm/intrinsics.rs` usando intrínsecos AVX-512 para procesar 256 Quds por ciclo."

- **Formato:** "Código Rust con comentarios de alto nivel. Usa `unsafe` solo cuando sea necesario."

- **Ejemplo:** "Sigue el estilo de la función `tensor\_collapse` existente."

### 3. **Flujo de Trabajo Recomendado**

1. **Activa Memory Bank:** Almacena decisiones arquitectónicas para persistencia entre sesiones.

2. **Usa Orchestrator Mode:** Para tareas complejas como la Fase 4 (`kilo --orchestrator "Implementa el backend de ADN"`).

3. **Itera con Especificaciones Claras:** Define metas, no-goals, archivos permitidos y pruebas de aceptación antes de cada tarea larga para evitar "improvisación" del agente.

4. **Revisa con `qa-reviewer`:** Usa el subagente de revisión para validar cambios críticos.

Esta configuración aprovecha las capacidades de Kilo Code de 2026 para un desarrollo agéntico eficiente y controlado.

