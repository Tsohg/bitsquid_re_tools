use re_core::byte_stream::{ByteStream, Stream};

use super::lua_values::{LuaValue, LuaTable, ArrayPart, HashPart};

pub struct PrototypeStream {
    stream: ByteStream,
}

impl PrototypeStream {
    pub fn new(stream: ByteStream) -> PrototypeStream {
        PrototypeStream { stream }
    }
    
    pub fn read_kn(&mut self) -> LuaValue {
        let mut kn_a = self.read_uleb();
        let is_a_double = (kn_a & 1) > 0;
        kn_a >>= 1;

        if is_a_double {
            let kn_b = self.read_uleb();
            let mut kn_union: u64 = kn_a as u64;
            kn_union <<= 16;
            kn_union |= kn_b as u64;
            return LuaValue::Double(kn_union as f64);
        } 
        else {
            return LuaValue::UInt(kn_a);
        }
    }

    pub fn read_kgc(&mut self) -> LuaValue {
        let type_byte = self.read_byte();
        match type_byte {
            0 => LuaValue::ChildProto, //signal that the prototyper needs to handle a child prototype by popping from the id stack and setting up parent/child relationship between the 2 prototypes.
            1 => LuaValue::Table(self.read_lua_table()), //add table constant -> array_part_len = uleb, hash_part_len = uleb, see TableConstant for more details.
            2 => LuaValue::SInt(self.read_uleb() as i32),
            3 => LuaValue::UInt(self.read_uleb()),
            4 => LuaValue::ComplexNum(self.read_complex_lua_number()),
            x => LuaValue::Str(self.read_lua_string((x - 5) as usize)),
        }
    }

    pub fn read_table_value(&mut self) -> LuaValue {
        let type_byte = self.read_byte();
        match type_byte {
            0 => LuaValue::Nil,
            1 => LuaValue::False,
            2 => LuaValue::True,
            3 => LuaValue::UInt(self.read_uleb()),
            4 => LuaValue::ComplexNum(self.read_complex_lua_number()),
            x => LuaValue::Str(self.read_lua_string((x - 5) as usize)),
        }
    }

    pub fn read_lua_table(&mut self) -> LuaTable {
        let array_part_len = self.read_uleb();
        let hash_part_len = self.read_uleb();
        let mut array_part = ArrayPart { values: Vec::new() };
        let mut hash_part = HashPart {
            keys: Vec::new(),
            values: Vec::new(),
        };
        self.read_table_array_part(&mut array_part, array_part_len as usize);
        self.read_table_hash_part(&mut hash_part, hash_part_len as usize);
        LuaTable::new(array_part, hash_part)
    }

    fn read_table_array_part(&mut self, array_part: &mut ArrayPart, len: usize) {
        for _ in 0..len {
            array_part.values.push(self.read_table_value());
        }
    }

    fn read_table_hash_part(&mut self, hash_part: &mut HashPart, len: usize) {
        for _ in 0..len {
            hash_part.keys.push(self.read_table_value());
            hash_part.values.push(self.read_table_value());
        }
    }

    fn read_complex_lua_number(&mut self) -> (u32, u32) {
        (self.read_uleb(), self.read_uleb()) //I think that it is in the form: XeY where X = first uleb, Y = second uleb. X may be a 32bit float and Y may be an integer.
    }

    fn read_lua_string(&mut self, len: usize) -> String {
        assert!(
            len > 0,
            "LjcReader::read_lua_string() -> Cannot read string length of 0 or less."
        );
        let utf8_string = self.stream.read(len);
        String::from_utf8(utf8_string).expect("Lua string could not be read.")
    }
}

impl Stream for PrototypeStream {
    fn read_byte(&mut self) -> u8 {
        self.stream.read_byte()
    }

    fn read_uint(&mut self) -> u32 {
        self.stream.read_uint()
    }

    fn read_ulong(&mut self) -> u64 {
        self.stream.read_ulong()
    }

    fn read(&mut self, len: usize) -> Vec<u8> {
        self.stream.read(len)
    }

    fn remaining_bytes(&self) -> usize {
        self.stream.remaining_bytes()
    }

    fn read_uleb(&mut self) -> u32 {
        self.stream.read_uleb()
    }

    fn peek_byte(&mut self) -> u8 {
        self.stream.peek_byte()
    }

    fn peek_bytes(&mut self, n: usize) -> Vec<u8> {
        self.stream.peek_bytes(n)
    }
}
