use crate::ast::*;
use crate::qir::{QirFunction, QirInstruction, QirModule};
use crate::qud::Qud;
use std::collections::HashMap;

/// Tipos cuaternarios soportados en QIR
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QirType {
    Qud,    // Valor cuaternario escalar
    VecQud, // Vector de valores cuaternarios
    Tensor, // Tensor multidimensional
}

/// Contexto de traducción con seguimiento de tipos y temporales
#[derive(Debug, Clone)]
struct QirContext {
    counter: usize,
    types: HashMap<String, QirType>,
    /// Mapeo de variables a registros para LetBind
    var_map: HashMap<String, String>,
}

impl QirContext {
    fn new() -> Self {
        QirContext {
            counter: 0,
            types: HashMap::new(),
            var_map: HashMap::new(),
        }
    }

    fn new_temp(&mut self) -> String {
        let name = format!("%t{}", self.counter);
        self.counter += 1;
        name
    }

    fn reserve_type(&mut self, name: &str, qtype: QirType) {
        self.types.insert(name.to_string(), qtype);
    }

    fn lookup_var(&self, name: &str) -> Option<&String> {
        self.var_map.get(name)
    }

    fn bind_var(&mut self, name: &str, reg: String) {
        self.var_map.insert(name.to_string(), reg);
    }
}

pub struct QirTranslator {
    simd_threshold: usize,
}

impl Default for QirTranslator {
    fn default() -> Self {
        Self::new()
    }
}

impl QirTranslator {
    pub fn new() -> Self {
        QirTranslator { simd_threshold: 8 }
    }

    /// Configura el umbral para activar operaciones SIMD
    pub fn with_simd_threshold(mut self, threshold: usize) -> Self {
        self.simd_threshold = threshold;
        self
    }

    pub fn translate_program(&self, program: &Program) -> QirModule {
        let mut functions = Vec::new();

        for func in &program.functions {
            let mut ctx = QirContext::new();
            // Registrar argumentos como variables
            for arg in &func.args {
                ctx.reserve_type(arg, QirType::Qud);
                ctx.bind_var(arg, arg.clone());
            }
            let body_ir = self.translate_expr(&func.body, &mut ctx);
            functions.push(QirFunction {
                name: func.name.clone(),
                args: func.args.clone(),
                body: body_ir,
            });
        }

        let mut ctx = QirContext::new();
        let main_ir = self.translate_expr(&program.main, &mut ctx);
        functions.push(QirFunction {
            name: "main".to_string(),
            args: Vec::new(),
            body: main_ir,
        });

        QirModule { functions }
    }

    fn translate_expr(&self, expr: &Expr, ctx: &mut QirContext) -> Vec<QirInstruction> {
        let mut ir = Vec::new();
        let _ = expr.to_qir_inst(&mut ir, ctx);
        ir.push(QirInstruction::Return("%dummy".to_string()));
        ir
    }
}

impl Expr {
    /// Traduce un Expr a instrucciones QIR, devolviendo el registro resultante
    fn to_qir_inst(&self, ir: &mut Vec<QirInstruction>, ctx: &mut QirContext) -> String {
        match self {
            Expr::QudLiteral(q) => {
                let name = ctx.new_temp();
                ir.push(QirInstruction::ConstQud(name.clone(), *q));
                ctx.reserve_type(&name, QirType::Qud);
                name
            }
            Expr::Var(name) => {
                // Buscar en var_map si existe binding
                ctx.lookup_var(name)
                    .cloned()
                    .unwrap_or_else(|| name.clone())
            }
            Expr::BinOp(left, op, right) => {
                let l = left.to_qir_inst(ir, ctx);
                let r = right.to_qir_inst(ir, ctx);
                let res = ctx.new_temp();
                let inst = match op {
                    BinOperator::QAdd => QirInstruction::QAdd(res.clone(), l.clone(), r.clone()),
                    BinOperator::QMul => QirInstruction::QMul(res.clone(), l.clone(), r.clone()),
                    BinOperator::QMax => {
                        // QMax: emitimos QAdd como placeholder, backend optimizará
                        QirInstruction::QAdd(res.clone(), l, r)
                    }
                    BinOperator::QMin => {
                        // QMin: emitimos QAdd como placeholder, backend optimizará
                        QirInstruction::QAdd(res.clone(), l, r)
                    }
                };
                ir.push(inst);
                ctx.reserve_type(&res, QirType::Qud);
                res
            }
            Expr::UnaryOp(op, operand) => {
                let val = operand.to_qir_inst(ir, ctx);
                let res = ctx.new_temp();
                match op {
                    UnaryOperator::QNot => ir.push(QirInstruction::QNot(res.clone(), val)),
                    // TODO: añadir QNeg (negación) si se implementa
                }
                ctx.reserve_type(&res, QirType::Qud);
                res
            }
            Expr::Collapse(inner) => {
                let val = inner.to_qir_inst(ir, ctx);
                let res = ctx.new_temp();
                ir.push(QirInstruction::Collapse(res.clone(), val));
                ctx.reserve_type(&res, QirType::Qud);
                res
            }
            Expr::LetBind(name, val_expr, body_expr) => {
                // Traducir valor y guardarlo
                let val_reg = val_expr.to_qir_inst(ir, ctx);
                ctx.bind_var(name, val_reg.clone());
                ctx.reserve_type(name, QirType::Qud);
                // Generar Store
                ir.push(QirInstruction::Store(name.to_string(), val_reg));
                // Continuar con el cuerpo
                body_expr.to_qir_inst(ir, ctx)
            }
            Expr::FuncCall(name, args) => {
                let mut arg_names = Vec::new();
                for arg in args {
                    arg_names.push(arg.to_qir_inst(ir, ctx));
                }
                let res = ctx.new_temp();
                ir.push(QirInstruction::Call(res.clone(), name.clone(), arg_names));
                ctx.reserve_type(&res, QirType::Qud);
                res
            }
            Expr::Lambda(_, _) => {
                // Lambda no soportado en QIR aún - devuelve Zero como placeholder
                let res = ctx.new_temp();
                ir.push(QirInstruction::ConstQud(res.clone(), Qud::Zero));
                ctx.reserve_type(&res, QirType::Qud);
                res
            }
        }
    }
}
