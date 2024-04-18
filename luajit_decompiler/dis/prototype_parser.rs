use std::collections::VecDeque;

use re_core::byte_stream::Stream;

use crate::dis::prototype::LuajitFileHeader;

use super::{prototype_stream::PrototypeStream, prototype::{Prototype, PrototypeHeader, UpValue, DebugInfoHeader, Constants}, lua_values::LuaValue, bytecode_instruction::Bci, ljc_header_reader::LJCHeaderReader};

pub struct PrototypeParser<'a> {
    proto_stream: &'a mut PrototypeStream,
    file_header: LuajitFileHeader,
    next_proto_id: usize,
    proto_id_stack: Vec<usize>,
}

impl<'a> PrototypeParser<'a> {
    pub fn new(proto_stream: &'a mut PrototypeStream) -> PrototypeParser {
        assert!(0x1b4c4a01 == u32::from_be_bytes(
            [
                proto_stream.read_byte(), proto_stream.read_byte(),
                proto_stream.read_byte(), proto_stream.read_byte()
            ]
        ), "Prototype stream is not beginning at the start of a luajit compiled file.");

        let dbg_flags = proto_stream.read_byte();
        let mut file_name: Option<String> = None;
        if dbg_flags == 0 {
            let file_name_len = proto_stream.read_uleb();
            let file_name_utf8 = proto_stream.read(file_name_len as usize);
            file_name = Some(String::from_utf8(file_name_utf8).expect("File name could not be read."));
        }

        let file_header = LuajitFileHeader {
            file_debug_flags: dbg_flags,
            file_name: file_name,
        };
        
        PrototypeParser { 
            proto_stream,
            file_header,
            next_proto_id: 0,
            proto_id_stack: vec![]
        }
    }

    pub fn parse(&mut self) -> Prototype {
        let header_reader = LJCHeaderReader::new(self.file_header.file_debug_flags as usize, self.next_prototype_id());

        let proto_header = header_reader.read_header(&mut self.proto_stream);
        let mut bcis = self.read_instructions(&proto_header);
        let uvs = self.read_raw_upvalues(&proto_header);
        let mut kgcs = self.read_kgcs(&proto_header);
        let kns = self.read_kns(&proto_header);
        let mut child_protos = self.get_child_prototypes(kgcs);   
        let mut constants = self.get_constants(kgcs);
        constants.non_strings.extend(kns);

        // let marks = Prototype::get_marked_instructions(&mut bcis);
        // Prototype::mark_unexpected_jmps_as_goto_or_iterj(&mut bcis, marks);

        Prototype {
            header: proto_header,
            uvs: uvs,
            constants: constants,
            symbols: symbols,
            instructions: bcis,
            proto_children: child_protos,
        }
    }

    fn next_prototype_id(&mut self) -> usize {
        self.proto_id_stack.push(self.next_proto_id);
        self.next_proto_id += 1;

        self.next_proto_id - 1
    }

    fn get_child_prototypes(&self, kgcs: Vec<LuaValue>) -> Vec<usize> {
        let mut child_protos: Vec<usize> = vec![];

        for kgc in kgcs.iter() {
            match kgc {
                LuaValue::ChildProto => child_protos.push(self.proto_id_stack.pop().unwrap()),
                _ => (),
            }
        }

        child_protos
    }

    fn get_constants(&self, kgcs: Vec<LuaValue>) -> Constants {
        let mut constants = Constants {
            strings: VecDeque::new(),
            non_strings: vec![],
        };

        for kgc in kgcs.iter() {
            match kgc {
                LuaValue::Str(s) => constants.strings.push_front(s.to_string()),
                _ => constants.non_strings.push(*kgc),
            }
        }

        constants
    }

    fn read_instructions(&mut self, prototype_header: &PrototypeHeader) -> Vec<Bci> {
        let mut bcis: Vec<Bci> = vec![];

        for i in 0..prototype_header.instruction_count {
            bcis.push(self.read_instruction(i as usize));
        }
        bcis
    }

    fn read_instruction(&self, index: usize) -> Bci {
        let instr_bytes = self.proto_stream.read(Bci::INSTRUCTION_SIZE as usize);
        Bci::new(
            index,
            instr_bytes[0], //op
            instr_bytes[1], //a
            instr_bytes[2], //c
            instr_bytes[3]  //b
        )
    }

    fn read_raw_upvalues(&mut self, prototype_header: &PrototypeHeader) -> Vec<UpValue> {
        let mut raw_uvs: Vec<UpValue> = vec![];

        for _ in 0..prototype_header.size_uv {
            raw_uvs.push(self.read_raw_upvalue());
        }
        raw_uvs
    }

    fn read_raw_upvalue(&mut self) -> UpValue{
        let uv = self.proto_stream.read(UpValue::UPVALUE_SIZE as usize);

        UpValue {
            table_index: uv[0],
            table_location: uv[1]
        }
    }

    fn read_kgcs(&mut self, prototype_header: &PrototypeHeader) -> Vec<LuaValue> {
        let mut kgcs: Vec<LuaValue> = vec![];

        for _ in 0..prototype_header.size_kgc {
            kgcs.push(self.proto_stream.read_kgc());
        }
        kgcs
    }

    fn read_kns(&mut self, prototype_header: &PrototypeHeader) -> Vec<LuaValue> {
        let mut kns: Vec<LuaValue> = vec![];

        for _ in 0..prototype_header.size_kn {
            kns.push(self.proto_stream.read_kn());
        }
        kns
    }
}