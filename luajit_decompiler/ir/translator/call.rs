use crate::{
    dis::bytecode_instruction::Bci,
    ir::{
        expressions::Exp,
    },
};

pub struct Call{}
impl Call {   
    pub fn call(bci: &Bci) -> Exp {
        //  [3] = print //get fname. usually GGET
        //  [4] = [1] //copy reference of variable(s) with MOV(s)
        //  [3](4..4) //arguments: (A+1...A+C-1) for CALL. slot 4 inclusive and 4 inclusive
        let a = bci.a() as u16;
        let b = bci.b() as u16;
        let c = bci.c() as u16;
        let d = bci.d();
        match bci.op {
            61 => Call::callm(bci),
            //CALL: A(A+!...A+C-1) but A+C for exclusive range.
            62 => Exp::Call(Box::new(Exp::Var(a)), 
                Box::new(Exp::Range((a+1) as u32, (a+c-1) as u32)), 
                Box::new(Exp::Range((a+1) as u32, (a+b-1) as u32))),
            63 => Exp::Return(Box::new(Call::callm(bci))),
            //CALLT: return A(A+1...A+D-1) but A+D for exclusive range.
            64 => Exp::Return(Box::new(Exp::Call(Box::new(Exp::Var(a)), 
                Box::new(Exp::Range((a+1) as u32, (a+d-1) as u32)), 
                Box::new(Exp::Range((a+1) as u32, (a+b-1) as u32))))),
            //ITERC/N is handled a lot similarly to FORI/L
            65 => Exp::Redundant("ITERC".to_string()),
            66 => Exp::Redundant("ITERN".to_string()),
            67 => Exp::VarArg(Box::new(Exp::Range((a+b-2) as u32, a as u32))), //a+b-2 -> a-1 inclusive, (a is varg slot)?.
            _  => Exp::Error("call".to_string()),
        }
    }

    fn callm(bci: &Bci) -> Exp {
        //CALLM has an additional param of '...' unless another CALLM is a
        // parameter to the current CALLM. In which case, give the varg to 
        // the nested CALLMs.

        //fn name in slot A. Slot A is reused as a return slot for fn if returning.
        //fixed returns go in slots: A to A+B (+1 exclusive?)
        //fixed params are slots: A+1 to A+C (+1 exclusive?)
        let a = bci.a() as u16;
        let c = bci.c() as u16;
        let b = bci.b() as u16;

        let f_name = Box::new(Exp::Var(a));
        let param_range = Box::new(Exp::Range((a+1) as u32, (a+c+1) as u32));
        let return_range = Box::new(Exp::Range(a as u32, (a+b) as u32));

        Exp::Call(f_name, param_range, return_range)
    }
}