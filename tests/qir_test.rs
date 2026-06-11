use quatra::qir::{Opcode, QirFunction, QirInstruction, QirModule};
use quatra::qud::Qud;
use std::io::Cursor;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_from_u8() {
        assert_eq!(Opcode::from_u8(0x01), Some(Opcode::ConstQud));
        assert_eq!(Opcode::from_u8(0x02), Some(Opcode::QAdd));
        assert_eq!(Opcode::from_u8(0x03), Some(Opcode::QMul));
        assert_eq!(Opcode::from_u8(0xFF), None);
    }

    #[test]
    fn test_const_qud_serialization() {
        let inst = QirInstruction::ConstQud("%t0".to_string(), Qud::One);
        let bytes = inst.to_bytes();
        assert_eq!(bytes[0], Opcode::ConstQud as u8);
        assert_eq!(bytes[1], 1); // Qud::One value
        assert_eq!(bytes[2], 3); // Name length
        assert_eq!(&bytes[3..6], b"%t0");
    }

    #[test]
    fn test_qadd_serialization() {
        let inst = QirInstruction::QAdd("%t2".to_string(), "%t0".to_string(), "%t1".to_string());
        let bytes = inst.to_bytes();
        assert_eq!(bytes[0], Opcode::QAdd as u8);
    }

    #[test]
    fn test_roundtrip_const_qud() {
        let original = QirInstruction::ConstQud("%t0".to_string(), Qud::Super);
        let bytes = original.to_bytes();
        let mut cursor = Cursor::new(&bytes);
        let decoded = QirInstruction::from_bytes(&mut cursor).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_roundtrip_qadd() {
        let original =
            QirInstruction::QAdd("%t2".to_string(), "%t0".to_string(), "%t1".to_string());
        let bytes = original.to_bytes();
        let mut cursor = Cursor::new(&bytes);
        let decoded = QirInstruction::from_bytes(&mut cursor).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_module_serialization() {
        let module = QirModule {
            functions: vec![QirFunction {
                name: "main".to_string(),
                args: vec![],
                body: vec![QirInstruction::ConstQud("%t0".to_string(), Qud::Zero)],
            }],
        };

        let bytes = module.to_binary();
        assert_eq!(bytes[0..3], [0x49, 0x52, 0x02]); // Magic + version
    }

    #[test]
    fn test_module_roundtrip() {
        let original = QirModule {
            functions: vec![
                QirFunction {
                    name: "add".to_string(),
                    args: vec!["a".to_string(), "b".to_string()],
                    body: vec![QirInstruction::QAdd(
                        "%t0".to_string(),
                        "a".to_string(),
                        "b".to_string(),
                    )],
                },
                QirFunction {
                    name: "main".to_string(),
                    args: vec![],
                    body: vec![QirInstruction::ConstQud("%t0".to_string(), Qud::One)],
                },
            ],
        };

        let bytes = original.to_binary();
        let mut cursor = Cursor::new(&bytes);
        let decoded = QirModule::from_binary(&mut cursor).unwrap();

        assert_eq!(decoded.functions.len(), original.functions.len());
        assert_eq!(decoded.functions[0].name, "add");
        assert_eq!(decoded.functions[0].args, vec!["a", "b"]);
    }

    #[test]
    fn test_simd_alignment_constant() {
        assert_eq!(quatra::qir::SIMD_ALIGNMENT, 32);
    }
}
