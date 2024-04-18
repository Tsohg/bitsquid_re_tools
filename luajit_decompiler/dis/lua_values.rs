use std::vec::Vec;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum LuaValue {
    Empty,
    Nil,
    ChildProto,
    Table(LuaTable),
    True,
    False,
    SInt(i32),
    UInt(u32),
    ComplexNum((u32, u32)),
    Str(String),
    Double(f64),
}

impl Default for LuaValue {
    fn default() -> LuaValue {
        LuaValue::Empty
    }
}

impl fmt::Display for LuaValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut v = "".to_string();
        match self {
            LuaValue::Empty             => (),
            LuaValue::Nil               => v = "nil".to_string(), 
            LuaValue::ChildProto        => (),
            LuaValue::Table(t)          => v = String::from(&format!("{}", t)),
            LuaValue::True              => v = "true".to_string(),
            LuaValue::False             => v = "false".to_string(),
            LuaValue::SInt(i)           => v = i.to_string(), 
            LuaValue::UInt(u)           => v = u.to_string(), 
            LuaValue::ComplexNum(n)     => v = String::from(&format!("{}+({})i", n.0, n.1)), 
            LuaValue::Str(s)            => v = String::from(s), 
            LuaValue::Double(d)         => v = d.to_string(), 
        }
        write!(f, "{}", v)
    }
}


#[derive(Debug)]
pub struct ArrayPart {
    pub values: Vec<LuaValue>,
}

#[derive(Debug)]
pub struct HashPart {
    pub keys: Vec<LuaValue>,
    pub values: Vec<LuaValue>,
}

#[derive(Debug)]
pub struct LuaTable {
    array_part: ArrayPart,
    hash_part: HashPart,
}

impl LuaTable {
    pub fn new(array_part: ArrayPart, hash_part: HashPart) -> LuaTable {
        LuaTable {
            array_part: array_part,
            hash_part: hash_part,
        }
    }
}
impl fmt::Display for LuaTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}
