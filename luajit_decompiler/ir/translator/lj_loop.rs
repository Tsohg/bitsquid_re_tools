use crate::{
    dis::bytecode_instruction::Bci,
    ir::expressions::Exp,
};

pub struct Loop{}
impl Loop {
    pub fn iter_jump(bci: &Bci) -> Exp {
        //Iter call is unknown at the moment, but the iter block is known.
        Exp::IterFor(Box::new(Exp::Empty), Box::new(Loop::loop_range(bci)))
    }

    pub fn iter_loop(_bci: &Bci) -> Exp {
        //Made redundant by iter_jump()
        Exp::Redundant("ITERL/IITERL/JITERL".to_string())
    }

    pub fn while_loop(bci: &Bci) -> Exp {
        Exp::While(Box::new(Exp::Empty), Box::new(Loop::loop_range(bci)))
    }

    fn loop_range(bci: &Bci) -> Exp {
        let start = (bci.index + 1) as u32;
        let end = (bci.get_jump_target() - 1) as u32;
        Exp::Range(start, end)
    }

    pub fn for_loop(bci: &Bci) -> Exp {
        //FORI denotes start of block for loop.
        //FORL is a backwards jump targeting the first instruction of the loop block.
        match bci.op {
            73 => Loop::fori(bci), //FORI
            74 => Exp::Error("JFORI unimplemented.".to_string()),
            75 => Exp::Redundant("FORL".to_string()), //FORLs are largely redundant information due to FORI.
            76 => Exp::Error("IFORL unimplemented.".to_string()),
            77 => Exp::Error("JFORL unimplemented.".to_string()),
            _  => Exp::Error("for_loop".to_string())
        }
    }

    fn fori(bci: &Bci) -> Exp {
        let a = bci.a() as u16;
        //Fori slots range from a to a+2 inclusive.
        let step = a+2;
        let stop = a+1;
        let start = a;

        //End of scope is INCLUSIVE.
        let scope_end = bci.get_jump_target();
        
        //Vars for now. Simplification is needed later because they are typically KSHORT (Num Expressions). 
        let step = Box::new(Exp::Var(step));
        let stop = Box::new(Exp::Var(stop));
        let start = Box::new(Exp::Var(start));
        //a is for loop itself. a+1 is the first instruction of the scope. 
        let scope = Box::new(Exp::Range((bci.index + 1) as u32, scope_end - 1));
        
        Exp::For(start, stop, step, scope)
    }
}