use crate::{
    dis::bytecode_instruction::Bci,
    ir::{
        expressions::Exp,
    },
};

pub struct Upvalue{}
impl Upvalue {
    pub fn upvalue(bci: &Bci) -> Exp {
        match bci.op {
            43      => Exp::Move(Box::new(Exp::Var(bci.a() as u16)), Box::new(Exp::Uv(bci.d()))),
            44..=47 => Upvalue::uset(bci),
            48      => Exp::UClo(bci.a() as u16, Box::new(Exp::Jump(bci.get_jump_target()))),
            _       => Exp::Error("uv".to_string()),
        }
    }

    fn uset(bci: &Bci) -> Exp {
        let a = Exp::Uv(bci.a() as u16);
        let d = match bci.op {
            44  => Exp::Var(bci.d()),
            45  => Exp::Str(bci.d()),
            46  => Exp::Num(bci.d()),
            47  => Exp::Pri(bci.d()),
            _   => Exp::Error("uset.d".to_string()),
        };
        let a = Box::new(a);
        let d = Box::new(d);

        Exp::Move(a, d)
    }
}