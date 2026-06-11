use crate::ast::*;
use crate::parser::Parser;
use crate::qud::Qud;
use std::collections::HashMap;

pub struct Interpreter {
    env: HashMap<String, Qud>,
    funcs: HashMap<String, FuncDef>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            env: HashMap::new(),
            funcs: HashMap::new(),
        }
    }

    pub fn eval(&mut self, expr: &Expr) -> Result<Qud, String> {
        match expr {
            Expr::QudLiteral(q) => Ok(*q),

            Expr::Var(name) => self
                .env
                .get(name)
                .copied()
                .ok_or_else(|| format!("Undefined variable: {}", name)),

            Expr::BinOp(left, op, right) => {
                let l = self.eval(left)?;
                let r = self.eval(right)?;
                match op {
                    BinOperator::QAdd => Ok(l + r),
                    BinOperator::QMul => Ok(l * r),
                    BinOperator::QMax => Ok(l.qmax(r)),
                    BinOperator::QMin => Ok(l.qmin(r)),
                }
            }

            Expr::UnaryOp(op, operand) => {
                let val = self.eval(operand)?;
                match op {
                    UnaryOperator::QNot => Ok(!val),
                }
            }

            Expr::Collapse(inner) => {
                let val = self.eval(inner)?;
                match val {
                    Qud::Super => Ok(Qud::One),
                    Qud::Error => Ok(Qud::Zero),
                    _ => Ok(val),
                }
            }

            Expr::LetBind(name, val_expr, body_expr) => {
                let val = self.eval(val_expr)?;
                self.env.insert(name.clone(), val);
                self.eval(body_expr)
            }

            Expr::FuncCall(name, args) => {
                let mut evaluated_args = Vec::new();
                for arg in args {
                    evaluated_args.push(self.eval(arg)?);
                }
                self.call_function(name, evaluated_args)
            }

            Expr::Lambda(_, _) => Err("Lambdas require type inference (Phase 2)".to_string()),
        }
    }

    pub fn register_function(&mut self, func: FuncDef) {
        self.funcs.insert(func.name.clone(), func);
    }

    pub fn call_function(&mut self, name: &str, _args: Vec<Qud>) -> Result<Qud, String> {
        let func = self
            .funcs
            .get(name)
            .ok_or_else(|| format!("Function not found: {}", name))?
            .clone();
        let prev_env: HashMap<String, Qud> = std::mem::take(&mut self.env);
        for (arg, val) in func.args.iter().zip(_args) {
            self.env.insert(arg.clone(), val);
        }
        let result = self.eval(&func.body);
        self.env = prev_env;
        result
    }

    pub fn eval_str(&mut self, input: &str) -> Result<Qud, String> {
        let mut parser = Parser::new(input)?;
        let program = parser.parse_program()?;
        self.eval(&program.main)
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}
