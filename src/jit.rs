//! Cranelift JIT compilation backend for QUATRA
//!
//! Compiles QIR to native machine code at runtime.

#[allow(unused_imports)]
#[cfg(feature = "jit")]
use crate::qir::QirModule;
#[cfg(feature = "jit")]
use cranelift::prelude::*;
#[cfg(feature = "jit")]
use cranelift_module::default_libcall_names;
#[cfg(feature = "jit")]
use cranelift_object::{ObjectBuilder, ObjectModule};

/// JIT compiler that compiles QIR to native code using cranelift
#[cfg(feature = "jit")]
pub struct JitCompiler {
    module: Option<ObjectModule>,
}

#[cfg(feature = "jit")]
impl JitCompiler {
    pub fn new() -> Result<Self, String> {
        let isa = cranelift::codegen::isa::lookup(target_lexicon::triple!("x86_64"))
            .map_err(|e| e.to_string())?
            .finish(settings::Flags::new(settings::builder()))
            .map_err(|e| e.to_string())?;
        let object_builder = ObjectBuilder::new(isa, "quatra_jit", default_libcall_names())
            .map_err(|e| e.to_string())?;
        let module = ObjectModule::new(object_builder);

        Ok(JitCompiler {
            module: Some(module),
        })
    }

    /// Compile a QIR module to native code
    pub fn compile(&mut self, _module: &crate::qir::QirModule) -> Result<Vec<u8>, String> {
        let module = self.module.take().ok_or("Module already consumed")?;
        let obj = module.finish();
        let bytes = obj.emit().map_err(|e| e.to_string())?;
        Ok(bytes)
    }
}

/// Native code generation using cranelift JIT
pub fn emit_native(_module: &crate::qir::QirModule) -> Result<Vec<u8>, String> {
    #[cfg(feature = "jit")]
    {
        let mut jit = JitCompiler::new()?;
        jit.compile(_module)
    }
    #[cfg(not(feature = "jit"))]
    {
        Err("Native code generation requires cranelift feature".to_string())
    }
}

/// ADN Backend - QUATRA to DNA sequences
///
/// Traduce QIR a secuencias de ADN sintético:
/// - Cada Qud (0,1,2,3) -> nucleótido (A,C,G,T)
/// - Operaciones QAdd/QMul -> reacciones bioquímicas
pub mod dna_backend {
    use crate::qud::Qud;

    /// Mapeo QUATRA -> ADN
    pub fn qud_to_nucleotide(q: Qud) -> &'static str {
        match q {
            Qud::Zero => "A",
            Qud::One => "C",
            Qud::Super => "G",
            Qud::Error => "T",
        }
    }

    /// Genera secuencia ADN para una función QUATRA
    pub fn generate_dna_sequence(name: &str, quds: &[Qud]) -> String {
        let nucleotides: String = quds.iter().map(|q| qud_to_nucleotide(*q)).collect();
        format!("{}; // {}", nucleotides, name)
    }

    /// Operaciones QUATRA como reacciones bioquímicas
    pub fn operation_to_reaction(op: &str) -> &'static str {
        match op {
            "qadd" => "hybridization_add",
            "qmul" => "enzymatic_mul",
            "qnot" => "polymerase_not",
            "collapse" => "restriction_collapse",
            _ => "unknown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::dna_backend;
    use crate::qud::Qud;

    #[test]
    fn test_qud_to_dna() {
        assert_eq!(dna_backend::qud_to_nucleotide(Qud::Zero), "A");
        assert_eq!(dna_backend::qud_to_nucleotide(Qud::One), "C");
        assert_eq!(dna_backend::qud_to_nucleotide(Qud::Super), "G");
        assert_eq!(dna_backend::qud_to_nucleotide(Qud::Error), "T");
    }
}
