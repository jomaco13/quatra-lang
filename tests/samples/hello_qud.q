-- hello_qud.q - Demo de operaciones cuaternarias
-- Operaciones básicas: adición, multiplicación, NOT, collapse

-- Función que demuestra qadd: 2 + 1 = 3 (Super + One = Error)
fun demo_qadd() =
  let a = 2 in
  let b = 1 in
  collapse (a <+> b)

-- Función que demuestra qmul: 2 * 1 = 2 (Super * One = Super)
fun demo_qmul() =
  let a = 2 in
  let b = 1 in
  collapse (a <*> b)

-- Función que demuestra qnot: ~~0 = 2 (rotación doble)
fun demo_qnot() =
  let x = 0 in
  collapse (~~x)

-- Entry point - demo completo
fun main() =
  let add_result = 2 <+> 1 in
  let mul_result = 2 <*> 1 in
  let not_result = ~~2 in
    collapse (add_result <+> mul_result)