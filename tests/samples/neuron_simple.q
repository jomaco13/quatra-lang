-- neuron_simple.q
-- Ejemplo: neurona cuaternaria simple

fun neurona(entrada) =
  let suma = entrada <+> 2 in
  collapse suma

fun main() =
  let x = 1 in
  neurona x