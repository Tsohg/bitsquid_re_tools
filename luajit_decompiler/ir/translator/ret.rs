use crate::{
    dis::bytecode_instruction::Bci,
    ir::{
        expressions::Exp,
    },
};

pub struct Ret{}
impl Ret {
    pub fn ret(bci: &Bci) -> Exp {
        let a = bci.a() as u16;
        let d = bci.d();
        match bci.op {
            69          => Exp::Return(Box::new(Exp::Range((a+d-2) as u32, a as u32))), //TODO: RETM???
            70          => Exp::Return(Box::new(Exp::Range((a+d-2) as u32, a as u32))), //RET
            71          => Exp::Return(Box::new(Exp::Empty)), //RET0
            72          => Exp::Return(Box::new(Exp::Var(a))), //RET1
            _           => Exp::Error("ret".to_string()),
        }
    }
}