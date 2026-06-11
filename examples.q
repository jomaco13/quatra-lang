-- examples.q - Ejemplos QUATRA

-- Ejemplo 1: Identity (0 → 0)
fun identity() = 0

-- Ejemplo 2: Rotation (One → Super → Error → Zero → One)
fun rotation() =
  let x = 1 in
  collapse (~~(~~(~~x)))

-- Ejemplo 3: Sumatoria cuaternaria
fun sum_demo() =
  let a = 3 in  -- Error
  let b = 2 in  -- Super
    collapse (a <+> b)

-- Entry: colapsa Error + Super = collapse(1) = One
fun main() =
  collapse (3 <+> 2)