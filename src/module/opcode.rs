#[derive(Debug, Eq, PartialEq)]
pub enum OpCode {
    Unreachable = 0x00, // trap immediately
    Nop = 0x01,         // no operation
    Block = 0x02,       //sig : block_type	begin a sequence of bytecodes, yielding 0 or 1 values
    Loop = 0x03,        //sig : block_type	begin a block which can also form control flow loops
    If = 0x04,          //sig : block_type	begin if bytecodes
    Else = 0x05,        // begin else bytecodes of if
    End = 0x0b,         // end a block, loop, or if
    Br = 0x0c,          //relative_depth : varuint32	break that targets an outer nested block
    BrIf = 0x0d, //relative_depth : varuint32	conditional break that targets an outer nested block
    BrTable = 0x0e, //see below	branch table control flow construct
    Return = 0x0f,

    // Call operators https://github.com/WebAssembly/design/blob/main/BinaryEncoding.md#call-operators-described-here
    Call = 0x10,
    CallIndirect = 0x11,

    // Parametric operators https://github.com/WebAssembly/design/blob/main/BinaryEncoding.md#parametric-operators-described-here
    Drop = 0x1a,
    Select = 0x1b,

    // Variable access https://github.com/WebAssembly/design/blob/main/BinaryEncoding.md#variable-access-described-here
    GetLocal = 0x20,
    SetLocal = 0x21,
    TeeLocal = 0x22,
    GetGlobal = 0x23,
    SetGlobal = 0x24,

    // Memory-related operators https://github.com/WebAssembly/design/blob/main/BinaryEncoding.md#memory-related-operators-described-here
    I32Load = 0x28,
    I64Load = 0x29,
    F32Load = 0x2a,
    F64Load = 0x2b,
    I32Load8S = 0x2c,
    I32Load8U = 0x2d,
    I32Load16S = 0x2e,
    I32Load16U = 0x2f,
    I64Load8S = 0x30,
    I64Load8U = 0x31,
    I64Load16S = 0x32,
    I64Load16U = 0x33,
    I64Load32S = 0x34,
    I64Load32U = 0x35,
    I32Store = 0x36,
    I64Store = 0x37,
    F32Store = 0x38,
    F64Store = 0x39,
    I32Store8 = 0x3a,
    I32Store16 = 0x3b,
    I64Store8 = 0x3c,
    I64Store16 = 0x3d,
    I64Store32 = 0x3e,
    CurrentMemory = 0x3f,
    GrowMemory = 0x40,

    // Constants https://github.com/WebAssembly/design/blob/main/BinaryEncoding.md#constants-described-here
    I32Const = 0x41,
    I64Const = 0x42,
    F32Const = 0x43,
    F64Const = 0x44,

    // Comparison operators  https://github.com/WebAssembly/design/blob/main/BinaryEncoding.md#comparison-operators-described-here
    I32Eqz = 0x45,
    I32Eq = 0x46,
    I32Ne = 0x47,
    I32LtS = 0x48,
    I32LtU = 0x49,
    I32GtS = 0x4a,
    I32GtU = 0x4b,
    I32LeS = 0x4c,
    I32LeU = 0x4d,
    I32GeS = 0x4e,
    I32GeU = 0x4f,
    I64Eqz = 0x50,
    I64Eq = 0x51,
    I64Ne = 0x52,
    I64LtS = 0x53,
    I64LtU = 0x54,
    I64GtS = 0x55,
    I64GtU = 0x56,
    I64LeS = 0x57,
    I64LeU = 0x58,
    I64GeS = 0x59,
    I64GeU = 0x5a,
    F32Eq = 0x5b,
    F32Ne = 0x5c,
    F32Lt = 0x5d,
    F32Gt = 0x5e,
    F32Le = 0x5f,
    F32Ge = 0x60,
    F64Eq = 0x61,
    F64Ne = 0x62,
    F64Lt = 0x63,
    F64Gt = 0x64,
    F64Le = 0x65,
    F64Ge = 0x66,

    I32Clz = 0x67,
    I32Ctz = 0x68,
    I32Popcnt = 0x69,
    I32Add = 0x6a,
    I32Sub = 0x6b,
    I32Mul = 0x6c,
    I32DivS = 0x6d,
    I32DivU = 0x6e,
    I32RemS = 0x6f,
    I32RemU = 0x70,
    I32And = 0x71,
    I32Or = 0x72,
    I32Xor = 0x73,
    I32Shl = 0x74,
    I32ShrS = 0x75,
    I32ShrU = 0x76,
    I32Rotl = 0x77,
    I32Rotr = 0x78,
    I64Clz = 0x79,
    I64Ctz = 0x7a,
    I64Popcnt = 0x7b,
    I64Add = 0x7c,
    I64Sub = 0x7d,
    I64Mul = 0x7e,
    I64DivS = 0x7f,
    I64DivU = 0x80,
    I64RemS = 0x81,
    I64RemU = 0x82,
    I64And = 0x83,
    I64Or = 0x84,
    I64Xor = 0x85,
    I64Shl = 0x86,
    I64ShrS = 0x87,
    I64ShrU = 0x88,
    I64Rotl = 0x89,
    I64Rotr = 0x8a,
    F32Abs = 0x8b,
    F32Neg = 0x8c,
    F32Ceil = 0x8d,
    F32Floor = 0x8e,
    F32Trunc = 0x8f,
    F32Nearest = 0x90,
    F32Sqrt = 0x91,
    F32Add = 0x92,
    F32Sub = 0x93,
    F32Mul = 0x94,
    F32Div = 0x95,
    F32Min = 0x96,
    F32Max = 0x97,
    F32Copysign = 0x98,
    F64Abs = 0x99,
    F64Neg = 0x9a,
    F64Ceil = 0x9b,
    F64Floor = 0x9c,
    F64Trunc = 0x9d,
    F64Nearest = 0x9e,
    F64Sqrt = 0x9f,
    F64Add = 0xa0,
    F64Sub = 0xa1,
    F64Mul = 0xa2,
    F64Div = 0xa3,
    F64Min = 0xa4,
    F64Max = 0xa5,
    F64Copysign = 0xa6,
}

impl OpCode {
    pub fn from_byte(byte: u8) -> OpCode {
        match byte {
            0x00 => OpCode::Unreachable,
            0x01 => OpCode::Nop,
            0x02 => OpCode::Block,
            0x03 => OpCode::Loop,
            0x04 => OpCode::If,
            0x05 => OpCode::Else,
            0x0b => OpCode::End,
            0x0c => OpCode::Br,
            0x0d => OpCode::BrIf,
            0x0e => OpCode::BrTable,
            0x0f => OpCode::Return,

            0x10 => OpCode::Call,
            0x11 => OpCode::CallIndirect,
            0x1a => OpCode::Drop,
            0x1b => OpCode::Select,

            0x20 => OpCode::GetLocal,
            0x21 => OpCode::SetLocal,
            0x22 => OpCode::TeeLocal,
            0x23 => OpCode::GetGlobal,
            0x24 => OpCode::SetGlobal,

            0x28 => OpCode::I32Load,
            0x29 => OpCode::I64Load,
            0x2a => OpCode::F32Load,
            0x2b => OpCode::F64Load,
            0x2c => OpCode::I32Load8S,
            0x2d => OpCode::I32Load8U,
            0x2e => OpCode::I32Load16S,
            0x2f => OpCode::I32Load16U,
            0x30 => OpCode::I64Load8S,
            0x31 => OpCode::I64Load8U,
            0x32 => OpCode::I64Load16S,
            0x33 => OpCode::I64Load16U,
            0x34 => OpCode::I64Load32S,
            0x35 => OpCode::I64Load32U,

            0x36 => OpCode::I32Store,
            0x37 => OpCode::I64Store,
            0x38 => OpCode::F32Store,
            0x39 => OpCode::F64Store,
            0x3a => OpCode::I32Store8,
            0x3b => OpCode::I32Store16,
            0x3c => OpCode::I64Store8,
            0x3d => OpCode::I64Store16,
            0x3e => OpCode::I64Store32,
            0x3f => OpCode::CurrentMemory,
            0x40 => OpCode::GrowMemory,

            0x41 => OpCode::I32Const,
            0x42 => OpCode::I64Const,
            0x43 => OpCode::F32Const,
            0x44 => OpCode::F64Const,

            0x45 => OpCode::I32Eqz,
            0x46 => OpCode::I32Eq,
            0x47 => OpCode::I32Ne,
            0x48 => OpCode::I32LtS,
            0x49 => OpCode::I32LtU,
            0x4a => OpCode::I32GtS,
            0x4b => OpCode::I32GtU,
            0x4c => OpCode::I32LeS,
            0x4d => OpCode::I32LeU,
            0x4e => OpCode::I32GeS,
            0x4f => OpCode::I32GeU,
            0x50 => OpCode::I64Eqz,
            0x51 => OpCode::I64Eq,
            0x52 => OpCode::I64Ne,
            0x53 => OpCode::I64LtS,
            0x54 => OpCode::I64LtU,
            0x55 => OpCode::I64GtS,
            0x56 => OpCode::I64GtU,
            0x57 => OpCode::I64LeS,
            0x58 => OpCode::I64LeU,
            0x59 => OpCode::I64GeS,
            0x5a => OpCode::I64GeU,
            0x5b => OpCode::F32Eq,
            0x5c => OpCode::F32Ne,
            0x5d => OpCode::F32Lt,
            0x5e => OpCode::F32Gt,
            0x5f => OpCode::F32Le,
            0x60 => OpCode::F32Ge,
            0x61 => OpCode::F64Eq,
            0x62 => OpCode::F64Ne,
            0x63 => OpCode::F64Lt,
            0x64 => OpCode::F64Gt,
            0x65 => OpCode::F64Le,
            0x66 => OpCode::F64Ge,

            // Conversions https://github.com/WebAssembly/design/blob/main/BinaryEncoding.md#conversions-described-here
            0x67 => OpCode::I32Clz,
            0x68 => OpCode::I32Ctz,
            0x69 => OpCode::I32Popcnt,
            0x6a => OpCode::I32Add,
            0x6b => OpCode::I32Sub,
            0x6c => OpCode::I32Mul,
            0x6d => OpCode::I32DivS,
            0x6e => OpCode::I32DivU,
            0x6f => OpCode::I32RemS,
            0x70 => OpCode::I32RemU,
            0x71 => OpCode::I32And,
            0x72 => OpCode::I32Or,
            0x73 => OpCode::I32Xor,
            0x74 => OpCode::I32Shl,
            0x75 => OpCode::I32ShrS,
            0x76 => OpCode::I32ShrU,
            0x77 => OpCode::I32Rotl,
            0x78 => OpCode::I32Rotr,
            0x79 => OpCode::I64Clz,
            0x7a => OpCode::I64Ctz,
            0x7b => OpCode::I64Popcnt,
            0x7c => OpCode::I64Add,
            0x7d => OpCode::I64Sub,
            0x7e => OpCode::I64Mul,
            0x7f => OpCode::I64DivS,
            0x80 => OpCode::I64DivU,
            0x81 => OpCode::I64RemS,
            0x82 => OpCode::I64RemU,
            0x83 => OpCode::I64And,
            0x84 => OpCode::I64Or,
            0x85 => OpCode::I64Xor,
            0x86 => OpCode::I64Shl,
            0x87 => OpCode::I64ShrS,
            0x88 => OpCode::I64ShrU,
            0x89 => OpCode::I64Rotl,
            0x8a => OpCode::I64Rotr,
            0x8b => OpCode::F32Abs,
            0x8c => OpCode::F32Neg,
            0x8d => OpCode::F32Ceil,
            0x8e => OpCode::F32Floor,
            0x8f => OpCode::F32Trunc,
            0x90 => OpCode::F32Nearest,
            0x91 => OpCode::F32Sqrt,
            0x92 => OpCode::F32Add,
            0x93 => OpCode::F32Sub,
            0x94 => OpCode::F32Mul,
            0x95 => OpCode::F32Div,
            0x96 => OpCode::F32Min,
            0x97 => OpCode::F32Max,
            0x98 => OpCode::F32Copysign,
            0x99 => OpCode::F64Abs,
            0x9a => OpCode::F64Neg,
            0x9b => OpCode::F64Ceil,
            0x9c => OpCode::F64Floor,
            0x9d => OpCode::F64Trunc,
            0x9e => OpCode::F64Nearest,
            0x9f => OpCode::F64Sqrt,
            0xa0 => OpCode::F64Add,
            0xa1 => OpCode::F64Sub,
            0xa2 => OpCode::F64Mul,
            0xa3 => OpCode::F64Div,
            0xa4 => OpCode::F64Min,
            0xa5 => OpCode::F64Max,
            0xa6 => OpCode::F64Copysign,

            _ => panic!("Invalid byte OpCode {} hex:{:x}", byte, byte),
        }
    }
}
