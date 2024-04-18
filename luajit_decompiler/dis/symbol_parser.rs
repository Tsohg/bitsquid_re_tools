use super::{prototype_stream::PrototypeStream, prototype::PrototypeHeader};

pub struct SymbolParser<'a> {
    proto_stream: &'a mut PrototypeStream,
    proto_header: &'a PrototypeHeader,
}
impl<'a>  SymbolParser<'a>  {
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
}