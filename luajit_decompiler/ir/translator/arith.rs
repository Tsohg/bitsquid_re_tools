use crate::{dis::bytecode_instruction::Bci, ir::expressions::Exp};

pub struct Arith{}
impl Arith {
    pub fn arith(bci: &Bci) -> Exp {
        let (a, b) = (Box::new(Exp::Var(bci.a() as u16)), Box::new(Exp::Var(bci.b() as u16)));
        let c;
        if (30..=34).contains(&bci.op) { //vv op
            c = Box::new(Exp::Var(bci.c() as u16));
        } else { //vn or nv
            c = Box::new(Exp::Num(bci.c() as u16));
        }

        if (25..=29).contains(&bci.op) { //nv
            Exp::Move(a, Box::new(Arith::binop(bci, c, b)))
        } else { //vx
            Exp::Move(a, Box::new(Arith::binop(bci, b, c)))
        }
    }

    fn binop(bci: &Bci, b: Box<Exp>, c: Box<Exp>) -> Exp {
        match bci.op % 5 {
            0                   => Exp::Add(b, c),
            1 if bci.op == 31   => Exp::Pow(b, c),
            1                   => Exp::Sub(b, c),
            2 if bci.op == 32   => Exp::Cat(b, c),
            2                   => Exp::Mul(b, c),
            3                   => Exp::Div(b, c),
            4                   => Exp::Mod(b, c),
            _                   => Exp::Error("binop".to_string()),
        }
    }
}