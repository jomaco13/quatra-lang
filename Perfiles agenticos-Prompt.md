Para codificar todo el plan de QUATRA con una IA, utiliza agentes especializados con prompts estructurados:

1. **Agentes y Perfiles:**

   - **Claude Code (Opus 4.7):** Ideal para razonamiento profundo y desarrollo complejo. Usa el perfil "Agente de Compilador" con contexto de Rust y lógica cuaternaria.

   - **Cursor:** Perfecto para desarrollo iterativo dentro de un IDE. Usa el perfil "Agente de VM" enfocado en rendimiento y SIMD.

   - **Codex (GPT-5.5):** Para automatización de tareas y generación de código rápido. Usa el perfil "Agente de Backend" para FPGAs y ADN.

2. **Prompts Agénticos Estructurados (Usa RCTFE):**

   - **Rol:** "Eres un ingeniero principal de compiladores especializado en lenguajes de bajo nivel."

   - **Contexto:** "Estás desarrollando QUATRA, un lenguaje funcional cuaternario. El código base está en Rust. La meta es el autoalojamiento."

   - **Tarea:** "Implementa la función `qadd\_simd` en `src/vm/intrinsics.rs` usando intrínsecos AVX-512 para procesar 256 Quds por ciclo."

   - **Formato:** "Código Rust con comentarios de alto nivel. Usa `unsafe` solo cuando sea necesario. Explica la lógica de empaquetado/desempaquetado."

   - **Ejemplo:** "Sigue el estilo de la función `tensor\_collapse` existente."

Este enfoque, basado en agentes especializados con prompts detallados, maximiza la precisión y eficiencia del desarrollo asistido por IA.

