use crate::{
    dis::bytecode_instruction::Bci,
    ir::{
        expressions::Exp,
    },
};

pub struct Unary{}
impl Unary {
    pub fn unary(bci: &Bci) -> Exp {
        let (a, d) = (Box::new(Exp::Var(bci.a() as u16)), Box::new(Exp::Var(bci.d())));
        match bci.op {
            16 => Exp::Move(a, d),
            17 => Exp::Move(a, Box::new(Exp::Not(d))),
            18 => Exp::Move(a, Box::new(Exp::Unm(d))),
            19 => Exp::Move(a, Box::new(Exp::Len(d))),
            _ => Exp::Error("unary".to_string()),
        }
    }
}