use quatra::optimizer::Optimizer;
use quatra::qir::{QirFunction, QirInstruction, QirModule};
use quatra::qud::Qud;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_folding_add() {
        let module = QirModule {
            functions: vec![QirFunction {
                name: "test".to_string(),
                args: vec![],
                body: vec![
                    QirInstruction::ConstQud("a".to_string(), Qud::One),
                    QirInstruction::ConstQud("b".to_string(), Qud::One),
                    QirInstruction::QAdd("c".to_string(), "a".to_string(), "b".to_string()),
                    QirInstruction::Return("c".to_string()),
                ],
            }],
        };

        let optimized = Optimizer::optimize(module);
        let body = &optimized.functions[0].body;

        // Should have folded a + b = One + One = Super
        assert!(matches!(&body[2], QirInstruction::ConstQud(_, Qud::Super)));
    }

    #[test]
    fn test_constant_folding_mul() {
        let module = QirModule {
            functions: vec![QirFunction {
                name: "test".to_string(),
                args: vec![],
                body: vec![
                    QirInstruction::ConstQud("a".to_string(), Qud::Super),
                    QirInstruction::ConstQud("b".to_string(), Qud::One),
                    QirInstruction::QMul("c".to_string(), "a".to_string(), "b".to_string()),
                    QirInstruction::Return("c".to_string()),
                ],
            }],
        };

        let optimized = Optimizer::optimize(module);
        let body = &optimized.functions[0].body;

        // Super * One = Super
        assert!(matches!(&body[2], QirInstruction::ConstQud(_, Qud::Super)));
    }

    #[test]
    fn test_constant_folding_not() {
        let module = QirModule {
            functions: vec![QirFunction {
                name: "test".to_string(),
                args: vec![],
                body: vec![
                    QirInstruction::ConstQud("a".to_string(), Qud::Zero),
                    QirInstruction::QNot("b".to_string(), "a".to_string()),
                    QirInstruction::Return("b".to_string()),
                ],
            }],
        };

        let optimized = Optimizer::optimize(module);
        let body = &optimized.functions[0].body;

        // QNot(Zero) = One
        assert!(matches!(&body[1], QirInstruction::ConstQud(_, Qud::One)));
    }

    #[test]
    fn test_constant_folding_collapse() {
        let module = QirModule {
            functions: vec![QirFunction {
                name: "test".to_string(),
                args: vec![],
                body: vec![
                    QirInstruction::ConstQud("a".to_string(), Qud::Super),
                    QirInstruction::Collapse("b".to_string(), "a".to_string()),
                    QirInstruction::Return("b".to_string()),
                ],
            }],
        };

        let optimized = Optimizer::optimize(module);
        let body = &optimized.functions[0].body;

        // Collapse(Super) = One
        assert!(matches!(&body[1], QirInstruction::ConstQud(_, Qud::One)));
    }
}