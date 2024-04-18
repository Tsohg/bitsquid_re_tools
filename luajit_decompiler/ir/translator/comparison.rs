use crate::{
    dis::bytecode_instruction::Bci,
    ir::{
        expressions::Exp,
    },
};

pub struct Comparison{}
impl Comparison {
    pub fn comparison(bci: &Bci) -> Exp {
        if (12..=15).contains(&bci.op) { //unary test/copy
            let a = if bci.op == 12 || bci.op == 13 {
                Box::new(Exp::Move(Box::new(Exp::Var(bci.a() as u16)), Box::new(Exp::Var(bci.d()))))
            } else {
                Box::new(Exp::Empty)
            };

            let mut d = Box::new(Exp::Var(bci.d()));
            let isf = bci.op == 13 || bci.op == 15;
            if isf {
                d = Box::new(Exp::Not(d));
            }
            Exp::IsT(a, d) //IsF = Not D. if it is a copy, A has a move instruction. if not copy, it is empty.

        } else {
            let a = Exp::Var(bci.a() as u16);
            let d = match bci.op {
                op if op < 6    => Exp::Var(bci.d()),
                op if op < 8    => Exp::Str(bci.d()),
                op if op < 10   => Exp::Num(bci.d()),
                op if op < 12   => Exp::Pri(bci.d()),
                _               => Exp::Error("comparison.d".to_string()),
            };
            let op = Comparison::comparison_op(bci);
            let a = Box::new(a);
            let d = Box::new(d);
            let op = Box::new(op);

            Exp::Comparison(a, op, d)
        }
    }

    fn comparison_op(bci: &Bci) -> Exp {
        match bci.op {
            0 if (bci.a() as u16) <= bci.d()            => Exp::Not(Box::new(Exp::Lt)),
            0 if (bci.a() as u16) > bci.d()             => Exp::Not(Box::new(Exp::Gt)),
            1 if (bci.a() as u16) <= bci.d()            => Exp::Lt,
            1 if (bci.a() as u16) > bci.d()             => Exp::Gt, 
            2 if (bci.a() as u16) <= bci.d()            => Exp::Not(Box::new(Exp::Lte)),
            2 if (bci.a() as u16) > bci.d()             => Exp::Not(Box::new(Exp::Gte)),
            3 if (bci.a() as u16) <= bci.d()            => Exp::Lte,
            3 if (bci.a() as u16) > bci.d()             => Exp::Gte,
            op if (4..=11).contains(&op) && op % 2 == 0 => Exp::Equals,
            op if (4..=11).contains(&op) && op % 2 == 1 => Exp::Not(Box::new(Exp::Equals)),
            _                                           => Exp::Error("comparison_op".to_string()),
        }
    }
}