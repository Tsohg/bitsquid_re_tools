use std::collections::VecDeque;

use re_core::byte_stream::{ByteStream, Stream};

use crate::dis::{bytecode_instruction::Bci, lua_values::*};

use super::prototype_stream::{PrototypeStream, self};

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
impl UpValue { const UPVALUE_SIZE: u8 = 2; }

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

pub struct Prototype {
    pub header: PrototypeHeader,
    pub uvs: Vec<UpValue>,
    pub constants: Constants,
    pub symbols: Vec<String>,
    pub instructions: Vec<Bci>,
    pub proto_children: Vec<usize>,
}

impl Prototype {
    pub fn new(ptr: &mut Prototyper, raw_prototype: Vec<u8>) -> Prototype {
        let mut proto_stream = PrototypeStream::new(ByteStream::new(raw_prototype));

        let header = Prototype::read_header(&mut proto_stream, &ptr);
        let mut bcis = Prototype::read_instructions(&mut proto_stream, &header);
        let uvs = Prototype::read_raw_upvalues(&mut proto_stream, &header);
        let mut kgcs = Prototype::read_kgcs(&mut proto_stream, &header);
        let kns = Prototype::read_kns(&mut proto_stream, &header);
        let (symbols, _line_info) = Prototype::read_debug_lines_and_symbols(&mut proto_stream, &header);

        let mut constants = Constants {
            strings: VecDeque::new(),
            non_strings: vec![],
        };

        let mut child_protos: Vec<usize> = vec![];

        { //Sort kgcs and kns into Constants type.
            for kgc in kgcs.iter_mut() {
                match kgc {
                    LuaValue::ChildProto => child_protos.push(ptr.proto_id_stack.pop().unwrap()),
                    LuaValue::Str(s) => constants.strings.push_front(s.to_string()),
                    _ => constants.non_strings.push(std::mem::take(kgc)),
                }
            }
            constants.non_strings.extend(kns);
        }

        { //Update id and id stack.
            let id = ptr.next_id;
            ptr.proto_id_stack.push(id);
            ptr.next_id += 1;
        }

        let marks = Prototype::get_marked_instructions(&mut bcis);
        Prototype::mark_unexpected_jmps_as_goto_or_iterj(&mut bcis, marks);

        Prototype {
            header: header,
            uvs: uvs,
            constants: constants,
            symbols: symbols,
            instructions: bcis,
            proto_children: child_protos,
        }
    }

    ///! Returns bytecode instructions that are marked as either Unexpected, Expeceted, or IterJ.
    fn get_marked_instructions(bcis: &Vec<Bci>) -> Vec<Mark> {
        //bci[i+1] is an expected jmp.
        //bci[bci[i+1].target - 1] is an expected jmp. (aka the target of the first expected jmp - 1)
        //Any unexpected JMP/UCLO is a goto.
        //Note: This does not catch ALL gotos in original source code,
        // but that is fine as equivalent code can still be reproduced without catching them all
        // as long as they pass the above expected JMP requirements.
        let mut marks: Vec<Mark> = vec![Mark::Unexpected; bcis.len()];

        for i in 0..bcis.len() {
            if bcis[i].op < 16 { //comparison ops.
                marks[i+1] = Mark::Expected;
                let target = (bcis[i+1].get_jump_target() - 1) as usize;
                marks[target] = Mark::Expected;

            } else if marks[i] == Mark::Unexpected && bcis[i].op == 84 { //JMP
                let target = (bcis[i].get_jump_target()) as usize;
                if bcis[target].op == 65 { //ITERC -> Expected JMPs can point to ITERC.
                    marks[i] = Mark::IterJ;
                }
            }
        }
        marks
    }

    ///! Changes bytecode instruction opcodes which are marked as Unexpected or IterJ that are also either JMP or UCLO instructions.
    fn mark_unexpected_jmps_as_goto_or_iterj(bcis: &mut Vec<Bci>, marks: Vec<Mark>) {
        for (i, m) in marks.iter().enumerate() {
            let is_jmp_or_uclo = bcis[i].op == 84 || bcis[i].op == 48;

            match *m {
                //Make unexpected JMP into a GOTO.
                Mark::Unexpected if is_jmp_or_uclo => bcis[i].op = 93,
                //Make JMP into IterJ.
                Mark::IterJ if is_jmp_or_uclo => bcis[i].op = 94,
                //Expected or conditional JMP instructions don't need changed.
                Mark::Expected => (),
                //Do nothing for the rest of the Unexpected instructions because otherwise, LOOP/FOR/FORI/etc... would be effected.
                _ => (),
            }
        }
    }

    ///! Read debug information from the prototype. These are the variable names.
    fn read_debug_lines_and_symbols(proto_stream: &mut PrototypeStream, header: &PrototypeHeader) -> (Vec<String>, Vec<u8>) {
        let mut symbols: Vec<String> = vec![];
        let mut line_nums: Vec<u8> = vec![];

        if let Some(dih) = &header.dbg_info_header {
            let dbg_info: Vec<u8> = proto_stream.read(dih.size_dbg as usize);
            let mut offset = 0;
            line_nums = Prototype::read_line_num_section(header, dih, &dbg_info, &mut offset);

            if offset < dbg_info.len() {
                symbols = Prototype::extract_symbols(&dbg_info, &mut offset);

            } else { symbols = Prototype::generate_symbols(header); }

        } else {
            symbols = Prototype::generate_symbols(header);
        }
        (symbols, line_nums)
    }

    ///! Read the debug line numbers. This contains information of which bytecode instructions belong on which line. 1:1 correspondence with BCIs.
    fn read_line_num_section(header: &PrototypeHeader, dih: &DebugInfoHeader, dbg_info: &Vec<u8>, offset: &mut usize) -> Vec<u8> {
        let entry_size = Prototype::line_entry_size(dih.num_lines);
        let line_sec_size = 1 + (entry_size * (header.instruction_count - 1)) as usize;
        *offset += line_sec_size;
        dbg_info[0..line_sec_size].to_vec()
    }

    ///! Extracts symbols (variable names) from its section after the line number section.
    fn extract_symbols(dbg_info: &Vec<u8>, offset: &mut usize) -> Vec<String> {
        let mut symbols: Vec<String> = vec![];
        loop {
            if *offset >= dbg_info.len() - 1 { break; } // +1 since this section terminates in 0x00.
            let sym = Prototype::extract_symbol(&dbg_info, offset);
            symbols.push(sym);
        }
        symbols
    }

    ///! Extract an individual symbol at the given offset.
    fn extract_symbol(dbg_info: &Vec<u8>, offset: &mut usize) -> String {
        let mut utf8: Vec<u8> = vec![];
        loop {
            if dbg_info[*offset] == 0 { break; }
            utf8.push(dbg_info[*offset]);
            *offset += 1;
        }
        *offset += 3; //skip null terminator + 2 unknown bytes. Unknown bytes *could* be 2 ulebs...not 100% sure. -> lj_debug.c/ line:172 -> line:176
        String::from_utf8(utf8).expect("Failed to convert symbol to utf8.")
    }

    ///! Determine the size of the entries, in number of bytes, in the line number section,
    fn line_entry_size(num_lines: u32) -> u32 {
        match num_lines {
            size if size < u8::MAX.into() => 1,
            size if size < u16::MAX.into() => 2,
            size if size < u32::MAX => 4,
            _ => panic!("Size of num_lines exceeds u32!"),
        }
    }

    ///! Generate symbols based on the prototype it was found in and its occurence in order. 
    fn generate_symbols(header: &PrototypeHeader) -> Vec<String> {
        let mut symbols: Vec<String> = Vec::new();
        for i in 0..header.frame_size {
            symbols.push(String::from(format!("var_pt{}_{}", header.id, i)));
        }
        symbols
    }

    ///! Read constant numbers from the prototype. Typically an Integer or Double number constant.
    fn read_kns(proto_stream: &mut PrototypeStream, header: &PrototypeHeader) -> Vec<LuaValue> {
        let mut kns: Vec<LuaValue> = vec![];

        for _ in 0..header.size_kn {
            kns.push(proto_stream.read_kn());
        }
        kns
    }

    /// Read global constant from the prototype. 
    /// KGCs can indicate parent/child relationship of prototypes or can be a table, u/sint, table, string, 
    /// or complex number.
    fn read_kgcs(proto_stream: &mut PrototypeStream, header: &PrototypeHeader) -> Vec<LuaValue> {
        let mut kgcs: Vec<LuaValue> = vec![];

        for _ in 0..header.size_kgc {
            kgcs.push(proto_stream.read_kgc());
        }
        kgcs
    }

    ///! Reads upvalues from the prototype that has not been bound to its corresponding symbol in a parent prototype.
    fn read_raw_upvalues(proto_stream: &mut PrototypeStream, header: &PrototypeHeader) -> Vec<UpValue> {
        let mut raw_uvs: Vec<UpValue> = vec![];

        for _ in 0..header.size_uv {
            raw_uvs.push(Prototype::read_raw_upvalue(proto_stream));
        }
        raw_uvs
    }

    ///! Read an individual upvalue from the prototype that has not been bound to its corresponding symbol in a parent prototype.
    fn read_raw_upvalue(proto_stream: &mut PrototypeStream) -> UpValue {
        let uv = proto_stream.read(UpValue::UPVALUE_SIZE as usize);

        UpValue {
            table_index: uv[0],
            table_location: uv[1]
        }
    }

    ///! Reads the bytecode instructions of the prototype.
    fn read_instructions(proto_stream: &mut PrototypeStream, header: &PrototypeHeader) -> Vec<Bci> {
        let mut bcis: Vec<Bci> = vec![];

        for i in 0..header.instruction_count {
            bcis.push(Prototype::read_instruction(proto_stream, i as usize));
        }
        bcis
    }

    ///! Reads a single bytecode instruction from the prototype.
    fn read_instruction(proto_stream: &mut PrototypeStream, index: usize) -> Bci {
        let instr_bytes = proto_stream.read_bytes(Bci::INSTRUCTION_SIZE as usize);
        Bci::new(
            index,
            instr_bytes[0], //op
            instr_bytes[1], //a
            instr_bytes[2], //c
            instr_bytes[3]  //b
        )
    }

    ///! Reads the prototype's header information at the beginning of a prototype.
    fn read_header(proto_stream: &mut PrototypeStream, ptr: &Prototyper) -> PrototypeHeader {
        let mut pth = PrototypeHeader {
            id: ptr.next_id,
            flags: proto_stream.read_byte(),
            num_params: proto_stream.read_byte(),
            frame_size: proto_stream.read_byte(),
            size_uv: proto_stream.read_byte(),
            size_kgc: proto_stream.read_uleb(),
            size_kn: proto_stream.read_uleb(),
            instruction_count: proto_stream.read_uleb(),
            dbg_info_header: None,
        };

        if ptr.ljfh.file_debug_flags & 0x02 == 0 {
            pth.dbg_info_header = Prototype::read_dbg_header(proto_stream);
        }
        pth
    }

    ///! Reads the debug info header of the prototype if it is present.
    fn read_dbg_header(proto_stream: &mut PrototypeStream) -> Option<DebugInfoHeader> {
        let dbg_size = proto_stream.read_uleb();
        if dbg_size > 0 {
            Some(DebugInfoHeader {
                size_dbg: dbg_size,
                first_line: proto_stream.read_uleb(),
                num_lines: proto_stream.read_uleb(),
            })
        } else { None }
    }
}

pub struct Prototyper {
    next_id: usize,
    reader: PrototypeStream,
    ljfh: LuajitFileHeader,
    proto_id_stack: Vec<usize>,
}

impl Prototyper {
    pub fn new(file_path: &str) -> Prototyper {
        let mut reader = LJFileReader::new(file_path);
        assert!(0x1b4c4a01 == u32::from_be_bytes(
            [
                reader.read_byte(), reader.read_byte(),
                reader.read_byte(), reader.read_byte()
            ]
        ));

        let dbg_flags = reader.read_byte();
        let mut file_name: Option<String> = None;
        if dbg_flags == 0 {
            let file_name_len = reader.read_uleb();
            file_name = Some(String::from_utf8(reader.read_bytes(file_name_len as usize)).expect("Original file name could not be read.").replace("@", ""));
        }

        let ljfh = LuajitFileHeader {
            file_debug_flags: dbg_flags,
            file_name: file_name,
        };
        
        Prototyper {
            next_id: 0,
            reader: reader,
            ljfh: ljfh,
            proto_id_stack: vec![],
        }
    }

    ///! Returns the next prototype in the compiled LuaJit File.
    pub fn next(&mut self) -> Option<Prototype> {
        let prototype_size = self.reader.read_uleb();
        if prototype_size > 0 {
            let raw = self.reader.read_bytes(prototype_size as usize);
            Some(Prototype::new(self, raw))

        } else { None }
    }
}