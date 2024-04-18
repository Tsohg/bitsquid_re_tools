use std::collections::VecDeque;

use super::{bytecode_instruction::Bci, lua_values::LuaValue};

pub struct Prototype {
    pub header: PrototypeHeader,
    pub uvs: Vec<UpValue>,
    pub constants: Constants,
    pub symbols: Vec<String>,
    pub instructions: Vec<Bci>,
    pub proto_children: Vec<usize>,
}

pub struct PrototypeHeader {
    pub id: usize,
    pub flags: u8,
    pub num_params: u8,
    pub frame_size: u8,
    pub size_uv: u8,
    pub size_kgc: u32,
    pub size_kn: u32,
    pub instruction_count: u32,
    pub dbg_info_header: Option<DebugInfoHeader>,
}

pub struct DebugInfoHeader {
    pub size_dbg: u32,
    pub first_line: u32,
    pub num_lines: u32,
}

pub struct UpValue {
    pub table_index: u8,
    pub table_location: u8,
}
impl UpValue { pub const UPVALUE_SIZE: u8 = 2; }

pub struct Constants {
    pub strings: VecDeque<String>, 
    pub non_strings: Vec<LuaValue>,
}

pub struct LuajitFileHeader {
    pub file_debug_flags: u8,
    pub file_name: Option<String>,
}

#[derive(PartialEq, Clone)]
enum Mark {
    Unexpected,
    Expected,
    IterJ,
}