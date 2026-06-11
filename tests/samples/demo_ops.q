-- demo_ops.q - Demostración de operaciones cuaternarias QUATRA

-- QAdd: Super + One = Error(3), luego collapse -> Zero
-- QMul: Super * One = Super(2), collapse -> One
-- QNot: ~~Zero = Super(2), collapse -> One
-- Resultado final: One

fun main() =
  let add_demo = collapse (2 <+> 1) in
  let mul_demo = collapse (2 <*> 1) in
  let not_demo = collapse (~~0) in
    add_demo <+> mul_demo