use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Registers {
    pub a: u8,
    pub c: u8,
    pub b: u8,
    pub d: u16,
}

impl Registers {
    pub fn new(a: u8, c: u8, b: u8) -> Registers {
        let mut d: u16 = b as u16;
        d <<= 8;
        d |= c as u16;
        Registers {
            a: a,
            c: c,
            b: b,
            d: d
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Bci {
    pub index: usize,
    pub op: u8,
    pub registers: Registers,
}

impl fmt::Display for Bci {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut target = "----".to_string();
        if self.is_jump() {
            target = self.get_jump_target().to_string();
        } else if self.op < 16 {
            target = format!("{}",self.index + 2)
        }
        write!(f, "{:>4}: [ {:>6} => A: [{:>3}], C: [{:>3}], B: [{:>3}], D: [{:>5}], JT: [{:>4}] ]",
        self.index,
        self.get_operation_name(), 
        self.registers.a, 
        self.registers.c, 
        self.registers.b, 
        self.registers.d,
        target
        )
    }
}

impl Bci {
    pub const INSTRUCTION_SIZE: u8 = 4;

    pub fn new(index: usize, op: u8, a: u8, c: u8, b: u8) -> Bci {
        Bci {
            index: index,
            op: op,
            registers: Registers::new(a, c, b),
        }
    }

    pub fn a(&self) -> u8   { self.registers.a }
    pub fn c(&self) -> u8   { self.registers.c }
    pub fn b(&self) -> u8   { self.registers.b }
    pub fn d(&self) -> u16  { self.registers.d }

    pub fn get_jump_target(&self) -> u32 {
        assert!(self.is_jump(), "Attempt to get jump target of bci that is not a jump: {}", self);
        1 + self.index as u32 + ((self.b() as u32) << 8 | self.c() as u32) - 0x8000
    }

    pub fn get_operation_name(&self) -> String {
        String::from(Bci::OP_LOOKUP[self.op as usize])
    }

    pub fn is_jump(&self) -> bool {
        match self.op {
            //0..=11 | //comparison
            //12..=15 | //unary test/copy
            48 | //UCLO
            68 | //ISNEXT
            73..=76 | //FOR
            78..=79 | //ITER
            81..=82 | //LOOP
            84 | //JMP
            93 | //GOTO
            94 => true, //ITERJ
            _ => false,
        }
    }


    pub const OP_LOOKUP: [&'static str; 95] = [
        "ISLT",
        "ISGE",
        "ISLE",
        "ISGT",
        "ISEQV",
        "ISNEV", //0-5 comparison V

        "ISEQS",
        "ISNES", //6-7 comparison S

        "ISEQN",
        "ISNEN", //8-9 comparison N
        
        "ISEQP",
        "ISNEP", //0-11 = conditional; 10-11 comparison P

        "ISTC",
        "ISFC",
        "IST",
        "ISF", //12-15 = unary test/copy

        "MOV",
        "NOT",
        "UNM",
        "LEN", //16-19 = unary

        "ADDVN",
        "SUBVN",
        "MULVN",
        "DIVVN",
        "MODVN", //20-24 = vn

        "ADDNV",
        "SUBNV",
        "MULNV",
        "DIVNV",
        "MODNV", //25-29 = nv

        "ADDVV",
        "SUBVV",
        "MULVV",
        "DIVVV",
        "MODVV", //30-34 = vv

        "POW", //31
        "CAT", //32

        "KSTR",
        "KCDATA",
        "KSHORT",
        "KNUM",
        "KPRI",
        "KNIL", //37-42 = constants

        "UGET",
        "USETV",
        "USETS",
        "USETN",
        "USETP",
        "UCLO", //43-48 = upvalue ops. UCLO is considered a JMP.

        "FNEW", //49

        "TNEW",
        "TDUP", 

        "GGET", 
        "GSET", //53
        "TGETV",
        "TGETS",
        "TGETB",
        "TSETV", //57
        "TSETS", //58
        "TSETB", //59
        "TSETM", //50-60 = table ops

        "CALLM",
        "CALL",
        "CALLMT",
        "CALLT",
        "ITERC",
        "ITERN",
        "VARG",
        "ISNEXT", //61-68 call and var-arg

        "RETM",
        "RET",
        "RET0",
        "RET1", //69-72 = ret

        "FORI",
        "JFORI",
        "FORL",
        "IFORL",
        "JFORL", //73-77 = for loop

        "ITERL",
        "IITERL",
        "JITERL", //78-80 = iter loop

        "LOOP",
        "ILOOP",
        "JLOOP", //81-83 = loop

        "JMP", //84

        "FUNCF",
        "IFUNCF",
        "JFUNCF",
        "FUNCV",
        "IFUNCV",
        "JFUNCV",
        "FUNCC",
        "FUNCCW", //85-92 funcs

        "GOTO", //93: Not part of the original LJ opcodes, but I added this here to rename unconditional jmp (and potentially UCLO) instructions as simply goto instructions.
        "ITERJ", //94: Not part of LJ opcodes. Used to mark JMPs which are associated with ITERC.
    ];
}
