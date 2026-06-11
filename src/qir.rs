use crate::qud::Qud;
use std::io::Read;

/// Opcode de instrucción QIR (1 byte cada una)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    ConstQud = 0x01,
    QAdd = 0x02,
    QMul = 0x03,
    QNot = 0x04,
    Collapse = 0x05,
    Branch = 0x06,
    BranchIf = 0x07,
    Jump = 0x08,
    Store = 0x09,
    Load = 0x0A,
    Call = 0x0B,
    Return = 0x0C,
    Label = 0x0D,
    Phi = 0x0E,
    // SIMD intrinsics
    QAddVec = 0x10,
    QMulVec = 0x11,
    QTranspose = 0x12,
    QDot = 0x13,
    // Control flow enhanced
    BranchTable = 0x14,
}

impl Opcode {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0x01 => Some(Opcode::ConstQud),
            0x02 => Some(Opcode::QAdd),
            0x03 => Some(Opcode::QMul),
            0x04 => Some(Opcode::QNot),
            0x05 => Some(Opcode::Collapse),
            0x06 => Some(Opcode::Branch),
            0x07 => Some(Opcode::BranchIf),
            0x08 => Some(Opcode::Jump),
            0x09 => Some(Opcode::Store),
            0x0A => Some(Opcode::Load),
            0x0B => Some(Opcode::Call),
            0x0C => Some(Opcode::Return),
            0x0D => Some(Opcode::Label),
            0x0E => Some(Opcode::Phi),
            0x10 => Some(Opcode::QAddVec),
            0x11 => Some(Opcode::QMulVec),
            0x12 => Some(Opcode::QTranspose),
            0x13 => Some(Opcode::QDot),
            0x14 => Some(Opcode::BranchTable),
            _ => None,
        }
    }

    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QirInstruction {
    /// ConstQud(dest, value) - Carga una constante Qud en el registro
    ConstQud(String, Qud),
    /// QAdd(dest, src1, src2) - adición cuaternaria modular
    QAdd(String, String, String),
    /// QMul(dest, src1, src2) - multiplicación cuaternaria
    QMul(String, String, String),
    /// QNot(dest, src) - operador NOT cuaternario (rotación)
    QNot(String, String),
    /// Collapse(dest, src) - colapso de Super/Error
    Collapse(String, String),
    /// Branch(label_true, label_false, condition) - branch condicional
    Branch(String, String, String),
    /// BranchIf(label_true, label_false, condition) - branch explícito si/no
    BranchIf(String, String, String),
    /// Jump(label) - salto incondicional
    Jump(String),
    /// Store(addr, value) - almacenar en memoria
    Store(String, String),
    /// Load(dest, addr) - cargar desde memoria
    Load(String, String),
    /// Call(dest, func, args) - llamada a función
    Call(String, String, Vec<String>),
    /// Return(value) - retorno de función
    Return(String),
    /// Label(name) - etiqueta de salto
    Label(String),
    /// Phi(dest, incoming) - operación phi para SSA
    Phi(String, Vec<String>),
    // SIMD intrinsics
    /// QAddVec(dest, src1, src2, len) - adición vectorial SIMD
    QAddVec(String, String, String, usize),
    /// QMulVec(dest, src1, src2, len) - multiplicación vectorial SIMD
    QMulVec(String, String, String, usize),
    /// QTranspose(dest, src, rows, cols) - transposición de matriz
    QTranspose(String, String, usize, usize),
    /// QDot(dest, a, b, len) - producto punto cuaternario
    QDot(String, String, String, usize),
    /// BranchTable(default, table_offset, key, targets) - branch por tabla
    BranchTable(String, usize, String, Vec<String>),
}

/// Configuración de alineación SIMD para operaciones vectoriales
pub const SIMD_ALIGNMENT: usize = 32; // 32 bytes para AVX2/AVX-512

impl QirInstruction {
    /// Serializa una instrucción a bytes QIR binary format
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        match self {
            QirInstruction::ConstQud(dest, val) => {
                buf.push(Opcode::ConstQud as u8);
                buf.extend_from_slice(&val.to_u8().to_le_bytes());
                buf.push(dest.len() as u8);
                buf.extend_from_slice(dest.as_bytes());
            }
            QirInstruction::QAdd(dest, a, b) => {
                buf.push(Opcode::QAdd as u8);
                push_varstr(&mut buf, a);
                push_varstr(&mut buf, b);
                push_varstr(&mut buf, dest);
            }
            QirInstruction::QMul(dest, a, b) => {
                buf.push(Opcode::QMul as u8);
                push_varstr(&mut buf, a);
                push_varstr(&mut buf, b);
                push_varstr(&mut buf, dest);
            }
            QirInstruction::QNot(dest, src) => {
                buf.push(Opcode::QNot as u8);
                push_varstr(&mut buf, dest);
                push_varstr(&mut buf, src);
            }
            QirInstruction::Collapse(dest, src) => {
                buf.push(Opcode::Collapse as u8);
                push_varstr(&mut buf, dest);
                push_varstr(&mut buf, src);
            }
            QirInstruction::Branch(lt, lf, cond) => {
                buf.push(Opcode::Branch as u8);
                push_varstr(&mut buf, lt);
                push_varstr(&mut buf, lf);
                push_varstr(&mut buf, cond);
            }
            QirInstruction::BranchIf(lt, lf, cond) => {
                buf.push(Opcode::BranchIf as u8);
                push_varstr(&mut buf, lt);
                push_varstr(&mut buf, lf);
                push_varstr(&mut buf, cond);
            }
            QirInstruction::Jump(label) => {
                buf.push(Opcode::Jump as u8);
                push_varstr(&mut buf, label);
            }
            QirInstruction::Store(addr, val) => {
                buf.push(Opcode::Store as u8);
                push_varstr(&mut buf, addr);
                push_varstr(&mut buf, val);
            }
            QirInstruction::Load(dest, addr) => {
                buf.push(Opcode::Load as u8);
                push_varstr(&mut buf, dest);
                push_varstr(&mut buf, addr);
            }
            QirInstruction::Call(dest, func, args) => {
                buf.push(Opcode::Call as u8);
                push_varstr(&mut buf, dest);
                push_varstr(&mut buf, func);
                buf.push(args.len() as u8);
                for arg in args {
                    push_varstr(&mut buf, arg);
                }
            }
            QirInstruction::Return(val) => {
                buf.push(Opcode::Return as u8);
                push_varstr(&mut buf, val);
            }
            QirInstruction::Label(name) => {
                buf.push(Opcode::Label as u8);
                push_varstr(&mut buf, name);
            }
            QirInstruction::Phi(dest, incoming) => {
                buf.push(Opcode::Phi as u8);
                push_varstr(&mut buf, dest);
                buf.push(incoming.len() as u8);
                for inc in incoming {
                    push_varstr(&mut buf, inc);
                }
            }
            QirInstruction::QAddVec(dest, a, b, len) => {
                buf.push(Opcode::QAddVec as u8);
                push_varstr(&mut buf, a);
                push_varstr(&mut buf, b);
                push_varstr(&mut buf, dest);
                buf.extend_from_slice(&len.to_le_bytes());
            }
            QirInstruction::QMulVec(dest, a, b, len) => {
                buf.push(Opcode::QMulVec as u8);
                push_varstr(&mut buf, a);
                push_varstr(&mut buf, b);
                push_varstr(&mut buf, dest);
                buf.extend_from_slice(&len.to_le_bytes());
            }
            QirInstruction::QTranspose(dest, src, rows, cols) => {
                buf.push(Opcode::QTranspose as u8);
                push_varstr(&mut buf, src);
                push_varstr(&mut buf, dest);
                buf.extend_from_slice(&rows.to_le_bytes());
                buf.extend_from_slice(&cols.to_le_bytes());
            }
            QirInstruction::QDot(dest, a, b, len) => {
                buf.push(Opcode::QDot as u8);
                push_varstr(&mut buf, a);
                push_varstr(&mut buf, b);
                push_varstr(&mut buf, dest);
                buf.extend_from_slice(&len.to_le_bytes());
            }
            QirInstruction::BranchTable(default, table_offset, key, targets) => {
                buf.push(Opcode::BranchTable as u8);
                push_varstr(&mut buf, key);
                buf.extend_from_slice(&table_offset.to_le_bytes());
                push_varstr(&mut buf, default);
                buf.push(targets.len() as u8);
                for t in targets {
                    push_varstr(&mut buf, t);
                }
            }
        }
        buf
    }

    /// Deserializa una instrucción desde bytes
    pub fn from_bytes<R: Read>(reader: &mut R) -> Option<Self> {
        let mut opcode_buf = [0u8; 1];
        reader.read_exact(&mut opcode_buf).ok()?;
        let opcode = Opcode::from_u8(opcode_buf[0])?;

        match opcode {
            Opcode::ConstQud => {
                let mut val_buf = [0u8; 1];
                reader.read_exact(&mut val_buf).ok()?;
                let len = read_varstr_len(reader)? as usize;
                let mut name_buf = vec![0u8; len];
                reader.read_exact(&mut name_buf).ok()?;
                let name = String::from_utf8(name_buf).ok()?;
                Some(QirInstruction::ConstQud(name, Qud::from_u8(val_buf[0])?))
            }
            Opcode::QAdd => {
                let a = read_varstr(reader)?;
                let b = read_varstr(reader)?;
                let dest = read_varstr(reader)?;
                Some(QirInstruction::QAdd(dest, a, b))
            }
            Opcode::QMul => {
                let a = read_varstr(reader)?;
                let b = read_varstr(reader)?;
                let dest = read_varstr(reader)?;
                Some(QirInstruction::QMul(dest, a, b))
            }
            Opcode::QNot => {
                let src = read_varstr(reader)?;
                let dest = read_varstr(reader)?;
                Some(QirInstruction::QNot(dest, src))
            }
            Opcode::Collapse => {
                let src = read_varstr(reader)?;
                let dest = read_varstr(reader)?;
                Some(QirInstruction::Collapse(dest, src))
            }
            Opcode::Branch => {
                let lt = read_varstr(reader)?;
                let lf = read_varstr(reader)?;
                let cond = read_varstr(reader)?;
                Some(QirInstruction::Branch(lt, lf, cond))
            }
            Opcode::BranchIf => {
                let lt = read_varstr(reader)?;
                let lf = read_varstr(reader)?;
                let cond = read_varstr(reader)?;
                Some(QirInstruction::BranchIf(lt, lf, cond))
            }
            Opcode::Jump => {
                let label = read_varstr(reader)?;
                Some(QirInstruction::Jump(label))
            }
            Opcode::Store => {
                let addr = read_varstr(reader)?;
                let val = read_varstr(reader)?;
                Some(QirInstruction::Store(addr, val))
            }
            Opcode::Load => {
                let dest = read_varstr(reader)?;
                let addr = read_varstr(reader)?;
                Some(QirInstruction::Load(dest, addr))
            }
            Opcode::Call => {
                let dest = read_varstr(reader)?;
                let func = read_varstr(reader)?;
                let arg_count = read_varstr_len(reader)? as usize;
                let mut args = Vec::new();
                for _ in 0..arg_count {
                    args.push(read_varstr(reader)?);
                }
                Some(QirInstruction::Call(dest, func, args))
            }
            Opcode::Return => {
                let val = read_varstr(reader)?;
                Some(QirInstruction::Return(val))
            }
            Opcode::Label => {
                let name = read_varstr(reader)?;
                Some(QirInstruction::Label(name))
            }
            Opcode::Phi => {
                let dest = read_varstr(reader)?;
                let inc_count = read_varstr_len(reader)? as usize;
                let mut incoming = Vec::new();
                for _ in 0..inc_count {
                    incoming.push(read_varstr(reader)?);
                }
                Some(QirInstruction::Phi(dest, incoming))
            }
            Opcode::QAddVec => {
                let a = read_varstr(reader)?;
                let b = read_varstr(reader)?;
                let dest = read_varstr(reader)?;
                let mut len_buf = [0u8; 8];
                reader.read_exact(&mut len_buf).ok()?;
                let len = u64::from_le_bytes(len_buf) as usize;
                Some(QirInstruction::QAddVec(dest, a, b, len))
            }
            Opcode::QMulVec => {
                let a = read_varstr(reader)?;
                let b = read_varstr(reader)?;
                let dest = read_varstr(reader)?;
                let mut len_buf = [0u8; 8];
                reader.read_exact(&mut len_buf).ok()?;
                let len = u64::from_le_bytes(len_buf) as usize;
                Some(QirInstruction::QMulVec(dest, a, b, len))
            }
            Opcode::QTranspose => {
                let src = read_varstr(reader)?;
                let dest = read_varstr(reader)?;
                let mut dims = [0u8; 16];
                reader.read_exact(&mut dims).ok()?;
                let rows = u64::from_le_bytes(dims[0..8].try_into().ok()?) as usize;
                let cols = u64::from_le_bytes(dims[8..16].try_into().ok()?) as usize;
                Some(QirInstruction::QTranspose(dest, src, rows, cols))
            }
            Opcode::QDot => {
                let a = read_varstr(reader)?;
                let b = read_varstr(reader)?;
                let dest = read_varstr(reader)?;
                let mut len_buf = [0u8; 8];
                reader.read_exact(&mut len_buf).ok()?;
                let len = u64::from_le_bytes(len_buf) as usize;
                Some(QirInstruction::QDot(dest, a, b, len))
            }
            Opcode::BranchTable => {
                let key = read_varstr(reader)?;
                let mut offset_buf = [0u8; 8];
                reader.read_exact(&mut offset_buf).ok()?;
                let offset = u64::from_le_bytes(offset_buf) as usize;
                let default = read_varstr(reader)?;
                let target_count = read_varstr_len(reader)? as usize;
                let mut targets = Vec::new();
                for _ in 0..target_count {
                    targets.push(read_varstr(reader)?);
                }
                Some(QirInstruction::BranchTable(default, offset, key, targets))
            }
        }
    }
}

fn push_varstr(buf: &mut Vec<u8>, s: &str) {
    buf.push(s.len() as u8);
    buf.extend_from_slice(s.as_bytes());
}

fn read_varstr<R: Read>(reader: &mut R) -> Option<String> {
    let len = read_varstr_len(reader)? as usize;
    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf).ok()?;
    String::from_utf8(buf).ok()
}

fn read_varstr_len<R: Read>(reader: &mut R) -> Option<u8> {
    let mut buf = [0u8; 1];
    reader.read_exact(&mut buf).ok()?;
    Some(buf[0])
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QirFunction {
    pub name: String,
    pub args: Vec<String>,
    pub body: Vec<QirInstruction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QirModule {
    pub functions: Vec<QirFunction>,
}

impl QirModule {
    pub fn dump(&self) {
        for func in &self.functions {
            println!("fun @{}({}) {{", func.name, func.args.join(", "));
            for inst in &func.body {
                println!("  {:?}", inst);
            }
            println!("}}\n");
        }
    }

    /// Serializa el módulo completo a formato binario QIR
    pub fn to_binary(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(0x49); // Magic: 'Q'
        buf.push(0x52); // Magic: 'R'
        buf.push(0x02); // Version: 2

        // Número de funciones
        buf.extend_from_slice(&(self.functions.len() as u64).to_le_bytes());

        for func in &self.functions {
            // Nombre de función
            push_varstr(&mut buf, &func.name);
            buf.push(func.args.len() as u8);
            for arg in &func.args {
                push_varstr(&mut buf, arg);
            }
            buf.extend_from_slice(&(func.body.len() as u64).to_le_bytes());
            for inst in &func.body {
                buf.extend(inst.to_bytes());
            }
        }
        buf
    }

    /// Deserializa un módulo desde formato binario
    pub fn from_binary<R: Read>(reader: &mut R) -> Option<Self> {
        let mut magic = [0u8; 3];
        reader.read_exact(&mut magic).ok()?;
        if magic != [0x49, 0x52, 0x02] {
            return None;
        }
        let mut func_count_buf = [0u8; 8];
        reader.read_exact(&mut func_count_buf).ok()?;
        let func_count = u64::from_le_bytes(func_count_buf) as usize;

        let mut functions = Vec::new();
        for _ in 0..func_count {
            let name = read_varstr(reader)?;
            let arg_count = read_varstr_len(reader)? as usize;
            let mut args = Vec::new();
            for _ in 0..arg_count {
                args.push(read_varstr(reader)?);
            }
            let mut inst_count_buf = [0u8; 8];
            reader.read_exact(&mut inst_count_buf).ok()?;
            let inst_count = u64::from_le_bytes(inst_count_buf) as usize;
            let mut body = Vec::new();
            for _ in 0..inst_count {
                body.push(QirInstruction::from_bytes(reader)?);
            }
            functions.push(QirFunction { name, args, body });
        }
        Some(QirModule { functions })
    }
}
