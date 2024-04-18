use re_core::byte_stream::Stream;

use super::{prototype::{PrototypeHeader, DebugInfoHeader}, prototype_stream::PrototypeStream};

pub struct LJCHeaderReader {
    proto_id: usize,
    file_debug_flags: usize,
}

impl LJCHeaderReader {
    pub fn new(file_debug_flags: usize, proto_id: usize) -> LJCHeaderReader {
        LJCHeaderReader { 
            proto_id,
            file_debug_flags,
        }
    }

    pub fn read_header(&self, proto_stream: &mut PrototypeStream) -> PrototypeHeader {
        let mut pth = PrototypeHeader {
            id: self.proto_id,
            flags: proto_stream.read_byte(),
            num_params: proto_stream.read_byte(),
            frame_size: proto_stream.read_byte(),
            size_uv: proto_stream.read_byte(),
            size_kgc: proto_stream.read_uleb(),
            size_kn: proto_stream.read_uleb(),
            instruction_count: proto_stream.read_uleb(),
            dbg_info_header: None,
        };

        if self.file_debug_flags & 0x02 == 0 {
            pth.dbg_info_header = self.read_debug_header(proto_stream);
        }
        pth
    }

    fn read_debug_header(&self, proto_stream: &mut PrototypeStream) -> Option<DebugInfoHeader> {
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