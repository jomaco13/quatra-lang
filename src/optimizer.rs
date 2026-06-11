use crate::qir::{QirInstruction, QirModule};

pub struct Optimizer;

impl Optimizer {
    pub fn optimize(mut module: QirModule) -> QirModule {
        for function in &mut module.functions {
            function.body = Self::fold_constants(function.body.clone());
        }
        module
    }

    fn fold_constants(body: Vec<QirInstruction>) -> Vec<QirInstruction> {
        let mut out = Vec::with_capacity(body.len());
        let mut constants: std::collections::HashMap<String, u8> = std::collections::HashMap::new();

        for inst in body {
            match inst {
                QirInstruction::ConstQud(name, value) => {
                    constants.insert(name.clone(), value.to_u8());
                    out.push(QirInstruction::ConstQud(name, value));
                }
                QirInstruction::QAdd(dest, a, b) => {
                    if let (Some(la), Some(lb)) = (constants.get(&a), constants.get(&b)) {
                        let folded = (la + lb) % 4;
                        constants.insert(dest.clone(), folded);
                        out.push(QirInstruction::ConstQud(dest, crate::qud::Qud::from_u8(folded).unwrap()));
                    } else {
                        out.push(QirInstruction::QAdd(dest, a, b));
                    }
                }
                other => out.push(other),
            }
        }

        out
    }
}
