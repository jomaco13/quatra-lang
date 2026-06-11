use crate::qir::{QirFunction, QirInstruction, QirModule};
use crate::qud::Qud;
use crate::vm::intrinsics;
use crate::vm::stack::QudStack;
use std::collections::HashMap;

/// Simulated memory heap for QUATRA VM
/// Uses a linear address space with 4-byte slots for Qud values
#[derive(Debug, Clone, Default)]
pub struct Memory {
    /// Heap storage: address -> Qud value
    data: HashMap<usize, Qud>,
    /// Next available address (simulated)
    next_addr: usize,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: HashMap::new(),
            next_addr: 0,
        }
    }

    /// Allocate space for a value, returns address
    pub fn alloc(&mut self, val: Qud) -> usize {
        let addr = self.next_addr;
        self.next_addr += 4;
        self.data.insert(addr, val);
        addr
    }

    /// Load value from address
    pub fn load(&self, addr: usize) -> Qud {
        self.data.get(&addr).copied().unwrap_or(Qud::Zero)
    }

    /// Store value at address
    pub fn store(&mut self, addr: usize, val: Qud) {
        self.data.insert(addr, val);
    }
}

pub struct ChimeraVM {
    stack: QudStack,
    registers: HashMap<String, Qud>,
    functions: HashMap<String, QirFunction>,
    intrinsics: HashMap<String, fn(&mut QudStack)>,
    /// Memory heap for dynamic allocation
    memory: Memory,
}

impl Default for ChimeraVM {
    fn default() -> Self {
        Self::new()
    }
}

impl ChimeraVM {
    pub fn new() -> Self {
        let mut vm = ChimeraVM {
            stack: QudStack::with_capacity(1024),
            registers: HashMap::new(),
            functions: HashMap::new(),
            intrinsics: HashMap::new(),
            memory: Memory::new(),
        };
        vm.register_intrinsics();
        vm
    }

    fn register_intrinsics(&mut self) {
        self.intrinsics
            .insert("qadd_simd".to_string(), intrinsics::qadd_simd);
        self.intrinsics
            .insert("qmul_simd".to_string(), intrinsics::qmul_simd);
        self.intrinsics
            .insert("tensor_collapse".to_string(), intrinsics::tensor_collapse);
    }

    pub fn load_module(&mut self, module: QirModule) {
        for func in module.functions {
            self.functions.insert(func.name.clone(), func);
        }
    }

    /// Helper to lookup value from register or stack
    #[allow(dead_code)]
    fn get_value(&self, name: &str) -> Option<Qud> {
        if let Some(&val) = self.registers.get(name) {
            return Some(val);
        }
        None
    }

    /// SIMD vectorized addition - operates on vector registers
    fn simd_add(&mut self, dest: &str, a: &str, b: &str, len: usize) {
        // For now, simulate element-wise addition
        // Real implementation would use AVX2/AVX-512 intrinsics
        let a_vals: Vec<Qud> = (0..len)
            .filter_map(|i| self.registers.get(&format!("{}_{}", a, i)).copied())
            .collect();
        let b_vals: Vec<Qud> = (0..len)
            .filter_map(|i| self.registers.get(&format!("{}_{}", b, i)).copied())
            .collect();

        for (i, (av, bv)) in a_vals.into_iter().zip(b_vals).enumerate() {
            self.registers.insert(format!("{}_{}", dest, i), av + bv);
        }
    }

    /// SIMD vectorized multiplication
    fn simd_mul(&mut self, dest: &str, a: &str, b: &str, len: usize) {
        let a_vals: Vec<Qud> = (0..len)
            .filter_map(|i| self.registers.get(&format!("{}_{}", a, i)).copied())
            .collect();
        let b_vals: Vec<Qud> = (0..len)
            .filter_map(|i| self.registers.get(&format!("{}_{}", b, i)).copied())
            .collect();

        for (i, (av, bv)) in a_vals.into_iter().zip(b_vals).enumerate() {
            self.registers.insert(format!("{}_{}", dest, i), av * bv);
        }
    }

    /// Matrix transpose: converts row-major to column-major layout
    fn simd_transpose(&mut self, dest: &str, src: &str, rows: usize, cols: usize) {
        for i in 0..rows {
            for j in 0..cols {
                let src_idx = format!("{}_{}_{}", src, i, j);
                let dest_idx = format!("{}_{}_{}", dest, j, i);
                if let Some(&val) = self.registers.get(&src_idx) {
                    self.registers.insert(dest_idx, val);
                }
            }
        }
    }

    /// Dot product for quaternary vectors
    fn simd_dot(&mut self, dest: &str, a: &str, b: &str, len: usize) -> Qud {
        let a_vals: Vec<Qud> = (0..len)
            .filter_map(|i| self.registers.get(&format!("{}_{}", a, i)).copied())
            .collect();
        let b_vals: Vec<Qud> = (0..len)
            .filter_map(|i| self.registers.get(&format!("{}_{}", b, i)).copied())
            .collect();

        let result: Qud = a_vals
            .into_iter()
            .zip(b_vals)
            .fold(Qud::Zero, |acc, (av, bv)| acc + av * bv);

        self.registers.insert(dest.to_string(), result);
        result
    }

    pub fn run(&mut self, entry_point: &str) -> Result<Qud, String> {
        let func = self
            .functions
            .get(entry_point)
            .cloned()
            .ok_or_else(|| format!("Function '{}' not found", entry_point))?;

        let instructions = &func.body;
        let mut ip = 0usize;
        let mut label_map: HashMap<String, usize> = HashMap::new();

        // Build label map for jumps
        for (idx, inst) in instructions.iter().enumerate() {
            if let QirInstruction::Label(name) = inst {
                label_map.insert(name.clone(), idx);
            }
        }

        while ip < instructions.len() {
            let inst = &instructions[ip];
            match inst {
                QirInstruction::ConstQud(dest, val) => {
                    self.registers.insert(dest.clone(), *val);
                }
                QirInstruction::QAdd(dest, a, b) => {
                    let a_val = self.registers.get(a).copied().unwrap_or(Qud::Zero);
                    let b_val = self.registers.get(b).copied().unwrap_or(Qud::Zero);
                    self.registers.insert(dest.clone(), a_val + b_val);
                }
                QirInstruction::QMul(dest, a, b) => {
                    let a_val = self.registers.get(a).copied().unwrap_or(Qud::Zero);
                    let b_val = self.registers.get(b).copied().unwrap_or(Qud::Zero);
                    self.registers.insert(dest.clone(), a_val * b_val);
                }
                QirInstruction::QNot(dest, src) => {
                    let src_val = self.registers.get(src).copied().unwrap_or(Qud::Zero);
                    self.registers.insert(dest.clone(), src_val.qnot());
                }
                QirInstruction::Collapse(dest, src) => {
                    let src_val = self.registers.get(src).copied().unwrap_or(Qud::Zero);
                    let collapsed = match src_val {
                        Qud::Super => Qud::One,
                        Qud::Error => Qud::Zero,
                        other => other,
                    };
                    self.registers.insert(dest.clone(), collapsed);
                }
                QirInstruction::Branch(lt, lf, cond) => {
                    let cond_val = self.registers.get(cond).copied().unwrap_or(Qud::Zero);
                    // Branch if condition != Zero (truthy)
                    if cond_val != Qud::Zero {
                        if let Some(&target) = label_map.get(lt) {
                            ip = target;
                            continue;
                        }
                    } else if let Some(&target) = label_map.get(lf) {
                        ip = target;
                        continue;
                    }
                }
                QirInstruction::BranchIf(lt, lf, cond) => {
                    let cond_val = self.registers.get(cond).copied().unwrap_or(Qud::Zero);
                    if cond_val == Qud::One {
                        if let Some(&target) = label_map.get(lt) {
                            ip = target;
                            continue;
                        }
                    } else if let Some(&target) = label_map.get(lf) {
                        ip = target;
                        continue;
                    }
                }
                QirInstruction::Jump(label) => {
                    if let Some(&target) = label_map.get(label) {
                        ip = target;
                        continue;
                    }
                }
                QirInstruction::Store(addr, val) => {
                    let val = self.registers.get(val).copied().unwrap_or(Qud::Zero);
                    // Try to parse as numeric address, otherwise use named address mapping
                    if let Ok(addr_num) = addr.parse::<usize>() {
                        self.memory.store(addr_num, val);
                    } else {
                        // For named addresses, store in registers (for compatibility)
                        // Allocations should use numeric addresses
                        self.registers.insert(addr.clone(), val);
                    }
                }
                QirInstruction::Load(dest, addr) => {
                    // Try to parse as numeric address, otherwise use named address mapping
                    if let Ok(addr_num) = addr.parse::<usize>() {
                        let val = self.memory.load(addr_num);
                        self.registers.insert(dest.clone(), val);
                    } else {
                        let val = self.registers.get(addr).copied().unwrap_or(Qud::Zero);
                        self.registers.insert(dest.clone(), val);
                    }
                }
                QirInstruction::Phi(dest, incoming) => {
                    // Phi node: select incoming value based on control flow
                    // For now, use the last available incoming value
                    for src in incoming {
                        if let Some(&val) = self.registers.get(src) {
                            self.registers.insert(dest.clone(), val);
                            break;
                        }
                    }
                }
                QirInstruction::Call(dest, func_name, args) => {
                    // Push arguments
                    for arg in args {
                        let val = self.registers.get(arg).copied().unwrap_or(Qud::Zero);
                        self.stack.push(val);
                    }

                    if let Some(intrinsic) = self.intrinsics.get(func_name) {
                        intrinsic(&mut self.stack);
                    } else if let Some(func) = self.functions.get(func_name).cloned() {
                        let result = self.run(&func.name)?;
                        self.registers.insert(dest.clone(), result);
                    } else {
                        return Err(format!("Unknown function/intrinsic: {}", func_name));
                    }
                }
                QirInstruction::Return(val) => {
                    return Ok(self.registers.get(val).copied().unwrap_or(Qud::Zero));
                }
                QirInstruction::QAddVec(dest, a, b, len) => {
                    self.simd_add(dest, a, b, *len);
                }
                QirInstruction::QMulVec(dest, a, b, len) => {
                    self.simd_mul(dest, a, b, *len);
                }
                QirInstruction::QTranspose(dest, src, rows, cols) => {
                    self.simd_transpose(dest, src, *rows, *cols);
                }
                QirInstruction::QDot(dest, a, b, len) => {
                    let result = self.simd_dot(dest, a, b, *len);
                    self.registers.insert(dest.to_string(), result);
                }
                QirInstruction::BranchTable(default, _table_offset, key, targets) => {
                    let key_val = self.registers.get(key).copied().unwrap_or(Qud::Zero);
                    let idx = (key_val.to_u8() as usize).min(targets.len() - 1);
                    if let Some(&target) = label_map.get(&targets[idx]) {
                        ip = target;
                        continue;
                    } else if let Some(&target) = label_map.get(default) {
                        ip = target;
                        continue;
                    }
                }
                _ => {}
            }
            ip += 1;
        }

        Ok(self.registers.values().next().copied().unwrap_or(Qud::Zero))
    }
}
