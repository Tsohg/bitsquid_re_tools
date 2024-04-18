use crate::{
    dis::bytecode_instruction::Bci,
    ir::{
        expressions::Exp,
    },
};

pub struct Constant{}
impl Constant {
    pub fn constant(bci: &Bci) -> Exp {
        let value = match bci.op {
            37 => Exp::Str(bci.d()),
            38 => Exp::Error("KCDATA is unimplemented.".to_string()),
            39 => Exp::Lit(bci.d()),
            40 => Exp::Var(bci.d()),
            41 => Exp::Pri(bci.d()),
            42 => Exp::Error("KNIL is unimplemented.".to_string()),
            _  => Exp::Error("constant.value".to_string()),
        };
        let dst = Box::new(Exp::Var(bci.a() as u16));
        let value = Box::new(value);
        Exp::Move(dst, value)
    }
}