use crate::qir::{QirInstruction, QirModule};
use crate::qud::Qud;
use std::collections::HashMap;

/// Optimization pass for QIR - constant folding and dead code elimination
pub struct Optimizer;

impl Optimizer {
    pub fn optimize(module: QirModule) -> QirModule {
        let mut optimized = module;

        for func in &mut optimized.functions {
            func.body = Self::fold_constants(func.body.clone());
        }

        optimized
    }

    /// Fold constant expressions: a + b where a,b are constants -> result
    fn fold_constants(body: Vec<QirInstruction>) -> Vec<QirInstruction> {
        let mut out = Vec::with_capacity(body.len());
        let mut constants: HashMap<String, Qud> = HashMap::new();

        for inst in body {
            match inst {
                QirInstruction::ConstQud(name, value) => {
                    constants.insert(name.clone(), value);
                    out.push(QirInstruction::ConstQud(name, value));
                }
                QirInstruction::QAdd(dest, a, b) => {
                    if let (Some(&la), Some(&lb)) = (constants.get(&a), constants.get(&b)) {
                        let folded = la + lb;
                        constants.insert(dest.clone(), folded);
                        out.push(QirInstruction::ConstQud(dest, folded));
                    } else {
                        let dest_clone = dest.clone();
                        out.push(QirInstruction::QAdd(dest, a, b));
                        constants.remove(&dest_clone);
                    }
                }
                QirInstruction::QMul(dest, a, b) => {
                    if let (Some(&la), Some(&lb)) = (constants.get(&a), constants.get(&b)) {
                        let folded = la * lb;
                        constants.insert(dest.clone(), folded);
                        out.push(QirInstruction::ConstQud(dest, folded));
                    } else {
                        let dest_clone = dest.clone();
                        out.push(QirInstruction::QMul(dest, a, b));
                        constants.remove(&dest_clone);
                    }
                }
                QirInstruction::QNot(dest, src) => {
                    if let Some(&val) = constants.get(&src) {
                        let folded = val.qnot();
                        constants.insert(dest.clone(), folded);
                        out.push(QirInstruction::ConstQud(dest, folded));
                    } else {
                        let dest_clone = dest.clone();
                        out.push(QirInstruction::QNot(dest, src));
                        constants.remove(&dest_clone);
                    }
                }
                QirInstruction::Collapse(dest, src) => {
                    if let Some(&val) = constants.get(&src) {
                        let collapsed = match val {
                            Qud::Super => Qud::One,
                            Qud::Error => Qud::Zero,
                            other => other,
                        };
                        constants.insert(dest.clone(), collapsed);
                        out.push(QirInstruction::ConstQud(dest, collapsed));
                    } else {
                        let dest_clone = dest.clone();
                        out.push(QirInstruction::Collapse(dest, src));
                        constants.remove(&dest_clone);
                    }
                }
                other => {
                    out.push(other);
                    constants.clear();
                }
            }
        }

        out
    }
}