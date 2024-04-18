use std::fmt;
use std::fmt::Formatter;

use crate::dis::bytecode_instruction::Bci;
use crate::dis::prototyper::Prototype;

pub struct InfixOp {
    pub opr1: String,
    pub op: String,
    pub opr2: String,
}

impl fmt::Display for InfixOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let s = String::from(&format!("{} {} {}", self.opr1, self.op, self.opr2));
        write!(f, "{}", s.trim_end())
    }
}

pub struct Statement {
    dst: String,
    infix: InfixOp,
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} := {}", self.dst, self.infix)
    }
}

pub struct StatementBuilder{}
impl StatementBuilder {
    pub fn build(bci: &Bci, pt: &Prototype) -> Option<Statement> {
        if bci.is_constant() {
            return Some(StatementBuilder::build_constant_assignment(bci, pt));
        } else if bci.is_table_op() {
            return Some(StatementBuilder::build_table_op(bci, pt));
        }
        None
    }

    fn build_table_op(bci: &Bci, pt: &Prototype) -> Statement {
        //bci.op -> 50..60 with 52 -> gget, 53 -> gset (global table ops)
        match bci.op {
            //52 => gget(bci, pt),
            
            _ => panic!("unimplemented table op: {}", bci),
        }
    }

    fn gget(bci: &Bci, pt: &Prototype) -> Statement {
        unimplemented!()
    }
    fn gset(bci: &Bci, pt: &Prototype) -> Statement {
        unimplemented!()
    }

    fn build_constant_assignment(bci: &Bci, pt: &Prototype) -> Statement {
        let dst = String::from(&pt.symbols.as_ref().unwrap()[bci.a() as usize]);
        let infix = InfixOp {
            opr1: String::from(&format!("{}", bci.d())),
            op: "".to_string(),
            opr2: "".to_string(),
        };
        Statement {
            dst: dst,
            infix: infix,
        }
    }
}