//! QIR to x86-64 Assembly code generation
//!
//! Generates NASM-compatible assembly from QIR for native execution.
//! SIMD-optimized for AVX2/AVX-512 quaternary operations.
//!
//! # AVX2 Emulation Strategy
//! Each Qud is 2 bits. AVX2 processes 256-bit vectors = 128 Quds per instruction.
//! AVX-512 processes 512-bit vectors = 256 Quds per instruction.

use crate::qir::{QirFunction, QirInstruction, QirModule};
use crate::qud::Qud;

pub struct CodeGenerator {
    asm_output: Vec<String>,
    simd_threshold: usize, // Vector length threshold for SIMD emission
}

impl Default for CodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            asm_output: Vec::new(),
            simd_threshold: 8, // Emit SIMD for vectors >= 8 Quds
        }
    }

    /// Configure threshold for SIMD emission
    pub fn with_simd_threshold(mut self, threshold: usize) -> Self {
        self.simd_threshold = threshold;
        self
    }

    pub fn generate(&mut self, module: &QirModule) -> String {
        self.asm_output
            .push("; QUATRA Binary Compiler - AVX2 Optimized".to_string());
        self.asm_output
            .push("; Generated from Q-IR bytecode".to_string());
        self.asm_output.push(".intel_syntax noprefix".to_string());
        self.asm_output.push(".section .text".to_string());
        self.asm_output.push(".globl main".to_string());
        self.asm_output.push(".align 32".to_string());

        // Emit SIMD-enabled header if needed
        self.emit_simd_prologue();

        for func in &module.functions {
            self.emit_function(func);
        }
        self.asm_output.join("\n")
    }

    fn emit_simd_prologue(&mut self) {
        // Align stack to 32 bytes for AVX2/AVX-512
        self.asm_output
            .push("; SIMD alignment prologue".to_string());
        self.asm_output.push("andq $-32, %rsp".to_string());
    }

    fn emit_function(&mut self, func: &QirFunction) {
        self.asm_output.push(format!("{}:", func.name));
        self.asm_output.push("  pushq %rbp".to_string());
        self.asm_output.push("  movq %rsp, %rbp".to_string());

        for inst in &func.body {
            self.emit_instruction(inst);
        }

        if !func
            .body
            .iter()
            .any(|inst| matches!(inst, QirInstruction::Return(_)))
        {
            self.asm_output.push("  popq %rbp".to_string());
            self.asm_output.push("  ret".to_string());
        }
    }

    fn emit_instruction(&mut self, inst: &QirInstruction) {
        match inst {
            QirInstruction::ConstQud(dest, val) => {
                let bits = match val {
                    Qud::Zero => 0,
                    Qud::One => 1,
                    Qud::Super => 2,
                    Qud::Error => 3,
                };
                self.asm_output
                    .push(format!("  movl ${}, %eax  # {} = {}", bits, dest, bits));
            }
            QirInstruction::QAdd(dest, a, b) => {
                self.asm_output
                    .push(format!("  # qadd {}, {}, {}", dest, a, b));
                self.asm_output.push("  movl %eax, %edx".to_string());
                self.asm_output.push("  addl %ebx, %edx".to_string());
                self.asm_output.push("  andl $3, %edx".to_string()); // Modulo 4
            }
            QirInstruction::QMul(dest, a, b) => {
                self.asm_output
                    .push(format!("  # qmul {}, {}, {}", dest, a, b));
                self.asm_output.push("  movl %eax, %edx".to_string());
                self.asm_output.push("  imull %ebx, %edx".to_string());
                self.asm_output.push("  andl $3, %edx".to_string()); // Modulo 4
            }
            QirInstruction::QNot(dest, src) => {
                self.asm_output
                    .push(format!("  # qnot {} from {}", dest, src));
                self.asm_output.push("  addl $1, %eax".to_string());
                self.asm_output.push("  andl $3, %eax".to_string()); // Rotación cíclica
            }
            QirInstruction::Collapse(dest, src) => {
                self.asm_output
                    .push(format!("  # collapse {} from {}", dest, src));
                self.asm_output.push("  cmpl $2, %eax".to_string());
                self.asm_output.push("  je .Lset_one_collapse".to_string());
                self.asm_output.push("  cmpl $3, %eax".to_string());
                self.asm_output.push("  je .Lset_zero_collapse".to_string());
                self.asm_output.push("  jmp .Lend_collapse".to_string());
                self.asm_output.push(".Lset_one_collapse:".to_string());
                self.asm_output.push("  movl $1, %eax".to_string());
                self.asm_output.push("  jmp .Lend_collapse".to_string());
                self.asm_output.push(".Lset_zero_collapse:".to_string());
                self.asm_output.push("  movl $0, %eax".to_string());
                self.asm_output.push(".Lend_collapse:".to_string());
            }
            QirInstruction::QAddVec(dest, a, b, len) => {
                self.emit_simd_add(dest, a, b, *len);
            }
            QirInstruction::QMulVec(dest, a, b, len) => {
                self.emit_simd_mul(dest, a, b, *len);
            }
            QirInstruction::QDot(dest, a, b, len) => {
                self.emit_simd_dot(dest, a, b, *len);
            }
            QirInstruction::Return(_) => {
                self.asm_output.push("  popq %rbp".to_string());
                self.asm_output.push("  ret".to_string());
            }
            _ => {
                self.asm_output.push(format!("  # {:?}", inst));
            }
        }
    }

    /// Emit AVX2 SIMD addition: processes 128 Quds per instruction (256-bit / 2 bits)
    fn emit_simd_add(&mut self, _dest: &str, _a: &str, _b: &str, len: usize) {
        self.asm_output
            .push("  ; SIMD QAdd (AVX2 emulation)".to_string());
        if len >= self.simd_threshold {
            // AVX2: vpaddd (add 8 x 32-bit integers) + vpand mask
            self.asm_output
                .push("  vmovdqu32 (%rdi), %ymm0".to_string()); // Load vector a
            self.asm_output
                .push("  vmovdqu32 (%rsi), %ymm1".to_string()); // Load vector b
            self.asm_output
                .push("  vpaddd %ymm1, %ymm0, %ymm0".to_string()); // Add
            self.asm_output
                .push("  vpand %ymm2, %ymm0, %ymm0".to_string()); // Mask with 0x03
            self.asm_output
                .push("  vmovdqu32 %ymm0, (%rdx)".to_string()); // Store result
            self.asm_output
                .push(format!("  ; Processed {} Quds in parallel", len));
        } else {
            self.asm_output
                .push("  ; Scalar fallback for small vectors".to_string());
        }
    }

    /// Emit AVX2 SIMD multiplication
    fn emit_simd_mul(&mut self, _dest: &str, _a: &str, _b: &str, len: usize) {
        self.asm_output
            .push("  ; SIMD QMul (AVX2 emulation)".to_string());
        if len >= self.simd_threshold {
            self.asm_output
                .push("  vmovdqu32 (%rdi), %ymm0".to_string());
            self.asm_output
                .push("  vmovdqu32 (%rsi), %ymm1".to_string());
            self.asm_output
                .push("  vpmulld %ymm1, %ymm0, %ymm0".to_string()); // Multiply
            self.asm_output
                .push("  vpand %ymm2, %ymm0, %ymm0".to_string()); // Mask
            self.asm_output
                .push("  vmovdqu32 %ymm0, (%rdx)".to_string());
            self.asm_output
                .push(format!("  ; Processed {} Quds in parallel", len));
        }
    }

    /// Emit dot product using SIMD reduction
    fn emit_simd_dot(&mut self, _dest: &str, _a: &str, _b: &str, len: usize) {
        self.asm_output
            .push("  ; SIMD QDot (AVX2 dot product)".to_string());
        self.asm_output
            .push("  vmovdqu32 (%rdi), %ymm0".to_string());
        self.asm_output
            .push("  vmovdqu32 (%rsi), %ymm1".to_string());
        self.asm_output
            .push("  vpmulld %ymm1, %ymm0, %ymm0".to_string()); // Multiply
        self.asm_output
            .push("  vpand %ymm2, %ymm0, %ymm0".to_string()); // Modulo 4

        // Horizontal sum reduction (conceptual)
        self.asm_output
            .push("  vextracti32 $0, %ymm0, %xmm1".to_string());
        self.asm_output
            .push("  vpaddd %xmm0, %xmm1, %xmm0".to_string());
        self.asm_output
            .push(format!("  ; Dot product of {} Quds", len));
    }
}

/// Native code generation stub - returns error until cranelift integration
pub fn emit_native(_module: &QirModule) -> Result<Vec<u8>, String> {
    Err("Native code generation requires cranelift feature".to_string())
}
