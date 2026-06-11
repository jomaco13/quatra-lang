-- quatra_showcase.q - Portfolio de QUATRA
-- Demostración de las 4 fases del lenguaje

-- Fase 1: Core - Tipos Qud básicos
fun zero_state() = 0
fun one_state() = 1
fun super_state() = 2
fun error_state() = 3

-- Fase 2: Q-IR - Operaciones
fun qud_adder(a, b) = collapse (a <+> b)

-- Fase 3: VM - Función con múltiples operaciones
fun tensor_rotation(x) =
  let y = ~~x in
  let z = ~~y in
  let w = ~~z in
    collapse w

-- Entry point
fun main() =
  let z = 0 in
    collapse (~~z)