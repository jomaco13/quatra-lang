-- matrix_mul.q (modo escalar demostrativo, Fase 2)
-- Multiplicacion escalar cuaternaria

fun mul(a, b) =
  a <*> b

fun main() =
  let x = 2
  let y = 3
  let res = mul(x, y)
  in collapse res