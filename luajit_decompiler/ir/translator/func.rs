use crate::{
    dis::bytecode_instruction::Bci,
    ir::{
        expressions::Exp,
    },
};

pub struct Func {}
impl Func {
    //Returns: A := func()
    pub fn fnew(bci: &Bci) -> Exp {
        let proto_index = bci.d();
        let func = Exp::Func(proto_index, Box::new(Exp::Empty));
        Exp::Move(Box::new(Exp::Var(bci.a() as u16)), Box::new(func))
    }
}