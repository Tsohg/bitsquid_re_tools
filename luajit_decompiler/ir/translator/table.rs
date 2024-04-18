use crate::{
    dis::bytecode_instruction::Bci,
    ir::{
        expressions::Exp,
    },
};

pub struct Table{}
impl Table {
    pub fn table(bci: &Bci) -> Exp {
        if bci.op == 60 { return Exp::Error("TSETM is unimplemented.".to_string()) }
    
        let a = Exp::Var(bci.a() as u16);
        
        if bci.op == 50 {
            return Exp::Move(Box::new(a), Box::new(Exp::Table(Box::new(Exp::Empty), Box::new(Exp::Empty))));
        } else if bci.op == 51 { return Exp::Error("TDUP is unimplemented.".to_string()) }
        
        let tbl;
        let is_global = (52..=53).contains(&bci.op);
        if is_global {
            let d = Box::new(Exp::Str(bci.d()));
            tbl = Exp::Table(Box::new(Exp::Global), d);
        } else {
            let b = Box::new(Exp::Var(bci.b() as u16));
            let c = match bci.op {
                54 | 57 => Box::new(Exp::Var(bci.c() as u16)),
                55 | 58 => Box::new(Exp::Str(bci.c() as u16)),
                56 | 59 => Box::new(Exp::Lit(bci.c() as u16)),
                _       => Box::new(Exp::Error("table.c".to_string())),
            };
            tbl = Exp::Table(b, c);
        }
    
        let is_set = bci.op == 53 || (57..=60).contains(&bci.op);
        if is_set {
            Exp::Move(Box::new(tbl), Box::new(a))
        } else {
            Exp::Move(Box::new(a), Box::new(tbl))
        }
    }
}