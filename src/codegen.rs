use crate::qir::{QirModule, QirInstruction, QirFunction};
use crate::qud::Qud;

pub struct CodeGenerator {
    asm_output: Vec<String>,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            asm_output: Vec::new(),
        }
    }

    pub fn generate(&mut self, module: &QirModule) -> String {
        self.emit_header();
        for func in &module.functions {
            self.emit_function(func);
        }
        self.asm_output.join("\n")
    }

    fn emit_header(&mut self) {
        self.asm_output.push(".section .text".to_string());
        self.asm_output.push(".globl main".to_string());
    }

    fn emit_function(&mut self, func: &QirFunction) {
        self.asm_output.push(format!("{}:", func.name));
        self.asm_output.push("  pushq %rbp".to_string());
        self.asm_output.push("  movq %rsp, %rbp".to_string());

        for inst in &func.body {
            match inst {
                QirInstruction::ConstQud(dest, val) => {
                    let bits = match val {
                        Qud::Zero => 0,
                        Qud::One => 1,
                        Qud::Super => 2,
                        Qud::Error => 3,
                    };
                    self.asm_output.push(format!("  movl ${}, %eax  # {} = {}", bits, dest, bits));
                }
                QirInstruction::QAdd(dest, a, b) => {
                    self.asm_output.push(format!("  # qadd {}, {}, {}", dest, a, b));
                    self.asm_output.push("  movl %eax, %edx".to_string());
                    self.asm_output.push("  addl %ebx, %edx".to_string());
                    self.asm_output.push("  andl $3, %edx".to_string());
                }
                QirInstruction::Collapse(dest, src) => {
                    self.asm_output.push(format!("  # collapse {} from {}", dest, src));
                    self.asm_output.push("  cmpl $2, %eax".to_string());
                    self.asm_output.push("  je set_one".to_string());
                    self.asm_output.push("  cmpl $3, %eax".to_string());
                    self.asm_output.push("  je set_zero".to_string());
                    self.asm_output.push("set_one:".to_string());
                    self.asm_output.push("  movl $1, %eax".to_string());
                    self.asm_output.push("  jmp end_collapse".to_string());
                    self.asm_output.push("set_zero:".to_string());
                    self.asm_output.push("  movl $0, %eax".to_string());
                    self.asm_output.push("end_collapse:".to_string());
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

        if !func.body.iter().any(|inst| matches!(inst, QirInstruction::Return(_))) {
            self.asm_output.push("  popq %rbp".to_string());
            self.asm_output.push("  ret".to_string());
        }
    }
}
