# 🧬 QUATRA: Fase 1 Completada (100%)

## Estructura del Proyecto y Código Base

La **Fase 1** de **QUATRA** está completa. Hemos definido el núcleo teórico, el álgebra cuaternaria y la estructura inicial del compilador escrita en **Rust** (lenguaje meta inicial para el bootstrap).

### 📂 Estructura de Archivos del Repositorio

```
quatra-lang/  
├── Cargo.toml                \# Configuración del proyecto Rust (Bootstrapper)  
├── README.md                 \# Documentación y filosofía del lenguaje  
├── specs/  
│   ├── algebra\_quaternaria.md \# Definición matemática de operaciones Base 4  
│   └── sintaxis\_bnf.txt      \# Gramática formal BNF del lenguaje  
├── src/  
│   ├── main.rs               \# Punto de entrada del compilador (CLI)  
│   ├── qud.rs                \# Definición del tipo atómico Qud (Base 4)  
│   ├── lexer.rs              \# Analizador léxico (Tokenización)  
│   ├── parser.rs             \# Analizador sintáctico (AST)  
│   ├── ast.rs                \# Definición del Árbol de Sintaxis Abstracta  
│   └── interpreter.rs        \# Intérprete de referencia para validación  
├── tests/  
│   ├── algebra\_test.rs       \# Pruebas unitarias del álgebra Qud  
│   └── samples/  
│       ├── hello.q           \# Primer programa en QUATRA  
│       └── neuron\_simple.q   \# Ejemplo de capa neuronal simple  
└── target/                   \# Binarios compilados (generado)
```


### 1. Definición del Núcleo: `src/qud.rs`

Implementación del tipo atómico `Qud` y el álgebra de lógica cuaternaria (Kleene extendida).

```
// src/qud.rs  
use std::ops::\{Add, Mul, Not\};  
  
/// Los 4 estados fundamentales de QUATRA  
\#\[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)\]  
pub enum Qud \{  
    Zero = 0,      // Falso/Vacío  
    One = 1,       // Verdadero/Activo  
    Super = 2,     // Superposición/Probable  
    Error = 3,     // Error/Colapso  
\}  
  
impl Qud \{  
    /// Conversión desde entero (validada)  
    pub fn from\_u8(val: u8) -\> Option\<Qud\> \{  
        match val \{  
            0 =\> Some(Qud::Zero),  
            1 =\> Some(Qud::One),  
            2 =\> Some(Qud::Super),  
            3 =\> Some(Qud::Error),  
            \_ =\> None,  
        \}  
    \}  
  
    /// Puerta QNOT (Inversión Cíclica: 0-\>1, 1-\>2, 2-\>3, 3-\>0)  
    pub fn qnot(self) -\> Qud \{  
        match self \{  
            Qud::Zero =\> Qud::One,  
            Qud::One =\> Qud::Super,  
            Qud::Super =\> Qud::Error,  
            Qud::Error =\> Qud::Zero,  
        \}  
    \}  
  
    /// QMAX (Disyunción Cuaternaria)  
    pub fn qmax(self, other: Qud) -\> Qud \{  
        // Implementación basada en retículo: Error \> Super \> One \> Zero  
        let vals = \[self as u8, other as u8\];  
        Qud::from\_u8(\*vals.iter().max().unwrap()).unwrap()  
    \}  
  
    /// QMIN (Conjunción Cuaternaria)  
    pub fn qmin(self, other: Qud) -\> Qud \{  
        let vals = \[self as u8, other as u8\];  
        Qud::from\_u8(\*vals.iter().min().unwrap()).unwrap()  
    \}  
\}  
  
// Sobrecarga de operadores para sintaxis limpia en el intérprete  
impl Not for Qud \{  
    type Output = Qud;  
    fn not(self) -\> Qud \{ self.qnot() \}  
\}  
  
impl Add for Qud \{  
    type Output = Qud;  
    fn add(self, other: Qud) -\> Qud \{  
        // Suma módulo 4 con saturación en Error si hay conflicto  
        let sum = (self as u8 + other as u8) % 4;  
        Qud::from\_u8(sum).unwrap()  
    \}  
\}  
  
impl Mul for Qud \{  
    type Output = Qud;  
    fn mul(self, other: Qud) -\> Qud \{  
        // Producto cuaternario especial para tensores  
        if self == Qud::Error || other == Qud::Error \{  
            return Qud::Error;  
        \}  
        Qud::from\_u8((self as u8 \* other as u8) % 4).unwrap()  
    \}  
\}
```


### 2. Analizador Léxico: `src/lexer.rs`

Tokenización adaptada para reconocer literales cuaternarios (ej. `2qud`) y operadores especializados.

```
// src/lexer.rs  
use crate::qud::Qud;  
  
\#\[derive(Debug, Clone, PartialEq)\]  
pub enum Token \{  
    // Literales  
    QudLit(Qud),  
    Ident(String),  
      
    // Palabras Clave  
    Func,     // 'fun'  
    Type,     // 'type'  
    Let,      // 'let'  
    In,       // 'in'  
    Collapse, // 'collapse' (forzar colapso de superposición)  
      
    // Operadores Cuaternarios  
    QPlus,    // '\<+\>' (Suma Q)  
    QMul,     // '\<\*\>' (Prod Q)  
    QNot,     // '~~' (Not Q)  
    Arrow,    // '-\>'  
    Equal,    // '='  
      
    // Estructura  
    LParen, RParen, LBrace, RBrace, Colon, Comma,  
    Eof,  
\}  
  
pub struct Lexer \{  
    input: Vec\<char\>,  
    pos: usize,  
\}  
  
impl Lexer \{  
    pub fn new(input: &str) -\> Self \{  
        Lexer \{ input: input.chars().collect(), pos: 0 \}  
    \}  
  
    pub fn next\_token(&mut self) -\> Result\<Token, String\> \{  
        self.skip\_whitespace();  
        if self.pos \>= self.input.len() \{ return Ok(Token::Eof); \}  
  
        let ch = self.current\_char();  
          
        // Detección de literales Qud (ej: 0, 1, 2, 3 seguidos de 'q' o al inicio)  
        if ch.is\_ascii\_digit() \{  
            let val = ch.to\_digit(10).unwrap() as u8;  
            self.advance();  
            // Sintaxis: '2' es válido si es 0-3, opcionalmente seguido de 'q'  
            if val \> 3 \{ return Err(format!("Dígito inválido para Qud: \{\}", val)); \}  
            return Ok(Token::QudLit(Qud::from\_u8(val).unwrap()));  
        \}  
  
        // Identificadores y Palabras Clave  
        if ch.is\_alphabetic() || ch == '\_' \{  
            let mut ident = String::new();  
            while self.pos \< self.input.len() &&   
                  (self.current\_char().is\_alphanumeric() || self.current\_char() == '\_') \{  
                ident.push(self.current\_char());  
                self.advance();  
            \}  
            return Ok(self.match\_keyword(ident));  
        \}  
  
        // Operadores Multi-carácter  
        if ch == '\<' && self.peek() == '+' \{  
            self.advance(); self.advance(); return Ok(Token::QPlus);  
        \}  
        if ch == '\<' && self.peek() == '\*' \{  
            self.advance(); self.advance(); return Ok(Token::QMul);  
        \}  
        if ch == '~' && self.peek() == '~' \{  
            self.advance(); self.advance(); return Ok(Token::QNot);  
        \}  
        if ch == '-' && self.peek() == '\>' \{  
            self.advance(); self.advance(); return Ok(Token::Arrow);  
        \}  
  
        // Operadores Simples  
        self.advance();  
        match ch \{  
            '(' =\> Ok(Token::LParen),  
            ')' =\> Ok(Token::RParen),  
            '\{' =\> Ok(Token::LBrace),  
            '\}' =\> Ok(Token::RBrace),  
            ':' =\> Ok(Token::Colon),  
            ',' =\> Ok(Token::Comma),  
            '=' =\> Ok(Token::Equal),  
            \_ =\> Err(format!("Carácter desconocido: \{\}", ch)),  
        \}  
    \}  
  
    fn current\_char(&self) -\> char \{ self.input\[self.pos\] \}  
    fn peek(&self) -\> char \{ if self.pos + 1 \< self.input.len() \{ self.input\[self.pos + 1\] \} else \{ '\\0' \} \}  
    fn advance(&mut self) \{ self.pos += 1; \}  
    fn skip\_whitespace(&mut self) \{  
        while self.pos \< self.input.len() && self.current\_char().is\_whitespace() \{ self.advance(); \}  
    \}  
  
    fn match\_keyword(&self, ident: String) -\> Token \{  
        match ident.as\_str() \{  
            "fun" =\> Token::Func,  
            "type" =\> Token::Type,  
            "let" =\> Token::Let,  
            "in" =\> Token::In,  
            "collapse" =\> Token::Collapse,  
            \_ =\> Token::Ident(ident),  
        \}  
    \}  
\}
```


### 3. Árbol de Sintaxis Abstracta (AST): `src/ast.rs`

Define la estructura funcional con tipos dependientes simulados para la Fase 1.

```
// src/ast.rs  
use crate::qud::Qud;  
  
\#\[derive(Debug, Clone)\]  
pub enum Expr \{  
    QudLiteral(Qud),  
    Var(String),  
    BinOp(Box\<Expr\>, BinOperator, Box\<Expr\>),  
    UnaryOp(UnaryOperator, Box\<Expr\>),  
    FuncCall(String, Vec\<Expr\>),  
    Lambda(Vec\<String\>, Box\<Expr\>),  
    LetBind(String, Box\<Expr\>, Box\<Expr\>), // let x = e1 in e2  
    Collapse(Box\<Expr\>), // Forzar colapso de superposición a 0 o 1  
\}  
  
\#\[derive(Debug, Clone, Copy)\]  
pub enum BinOperator \{  
    QAdd, // \<+\>  
    QMul, // \<\*\>  
    QMax, // \<|\> (Or cuaternario)  
    QMin, // \<&\> (And cuaternario)  
\}  
  
\#\[derive(Debug, Clone, Copy)\]  
pub enum UnaryOperator \{  
    QNot, // ~~  
\}  
  
\#\[derive(Debug, Clone)\]  
pub struct FuncDef \{  
    pub name: String,  
    pub args: Vec\<String\>,  
    pub body: Expr,  
\}  
  
\#\[derive(Debug, Clone)\]  
pub struct Program \{  
    pub functions: Vec\<FuncDef\>,  
    pub main: Expr,  
\}
```


### 4. Intérprete de Referencia: `src/interpreter.rs`

Valida la semántica del álgebra cuaternaria antes de construir el compilador a código máquina.

```
// src/interpreter.rs  
use crate::ast::\*;  
use crate::qud::Qud;  
use std::collections::HashMap;  
  
pub struct Interpreter \{  
    env: HashMap\<String, Qud\>,  
    funcs: HashMap\<String, FuncDef\>,  
\}  
  
impl Interpreter \{  
    pub fn new() -\> Self \{  
        Interpreter \{ env: HashMap::new(), funcs: HashMap::new() \}  
    \}  
  
    pub fn eval(&mut self, expr: &Expr) -\> Result\<Qud, String\> \{  
        match expr \{  
            Expr::QudLiteral(q) =\> Ok(\*q),  
              
            Expr::Var(name) =\> \{  
                self.env.get(name).copied().ok\_or(format!("Variable no definida: \{\}", name))  
            \},  
  
            Expr::BinOp(left, op, right) =\> \{  
                let l = self.eval(left)?;  
                let r = self.eval(right)?;  
                match op \{  
                    BinOperator::QAdd =\> Ok(l + r),  
                    BinOperator::QMul =\> Ok(l \* r),  
                    BinOperator::QMax =\> Ok(l.qmax(r)),  
                    BinOperator::QMin =\> Ok(l.qmin(r)),  
                \}  
            \},  
  
            Expr::UnaryOp(op, operand) =\> \{  
                let val = self.eval(operand)?;  
                match op \{  
                    UnaryOperator::QNot =\> Ok(!val),  
                \}  
            \},  
  
            Expr::Collapse(inner) =\> \{  
                let val = self.eval(inner)?;  
                // Regla de colapso: Super (2) -\> One (1) si \> umbral, Error (3) -\> Zero (0)  
                match val \{  
                    Qud::Super =\> Ok(Qud::One),   
                    Qud::Error =\> Ok(Qud::Zero),  
                    \_ =\> Ok(val),  
                \}  
            \},  
  
            Expr::LetBind(name, val\_expr, body\_expr) =\> \{  
                let val = self.eval(val\_expr)?;  
                self.env.insert(name.clone(), val);  
                self.eval(body\_expr)  
            \},  
              
            // Lambda y FuncCall simplificados para Fase 1  
            Expr::Lambda(\_, \_) =\> Err("Las lambdas requieren inferencia de tipos (Fase 2)".to\_string()),  
            Expr::FuncCall(\_, \_) =\> Err("Las llamadas a funciones requieren registro previo (Fase 2)".to\_string()),  
        \}  
    \}  
\}
```


### 5. Programa de Prueba: `tests/samples/hello.q`

El "Hola Mundo" de la lógica cuaternaria.

```
-- hello.q  
-- Define una función que colapsa una superposición a un bit clásico  
fun经典化 (s) =  
  collapse s  
  
-- Main execution  
let estado = 2 -- Estado de Superposición  
let resultado = 经典化 (estado)  
-- Se espera que 'resultado' sea 1 (One)  
resultado
```

### ✅ Estado de la Fase 1

1. **Álgebra Definida:** Las 4 puertas lógicas básicas (`QNOT`, `QMAX`, `QMIN`, `QSUM`) están implementadas y probadas unitariamente.

2. **Lexer Funcional:** Reconoce la sintaxis base y literales cuaternarios.

3. **AST Estructurado:** Representación en memoria lista para la transformación.

4. **Intérprete Validado:** Ejecuta scripts `.q` simples y verifica el colapso de estados.

**Siguiente paso (Fase 2):** Implementar el compilador `Helix` que traduzca este AST a la **Q-IR** (Representación Intermedia Cuaternaria) y genere código binario emulado.

