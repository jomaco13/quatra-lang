-- neuron_simple.q
-- Ejemplo: capa neuronal simple en QUATRA
-- Suma ponderada de activaciones y colapso final

fun neurona(entrada, peso, sesgo) =
  let suma = entrada <+> peso
  let activacion = suma <+> sesgo
  collapse activacion

let x = 1
let w = 2
let b = 1
let salida = neurona(x, w, b)
salida
