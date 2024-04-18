use std::fmt;

pub enum Exp { //Expression.
    Error(String),
    Empty,
    Redundant(String),

    //Pain
    Goto(u32), //jmp target.

    //Slots
    Var(u16),

    //Slot Range. Replace anything that uses a range of bcis to an exp
    // that represents a block?
    Range(u32, u32),

    //Constants
    Num(u16),   //index into number constant table.
    Lit(u16),   //literal number not index.
    Str(u16),   //slot into the Strings table
    Uv(u16),    //slot into the uv table.
    Pri(u16),   //primitive literal such as nil, false, true -> 0, 1, 2.
    //Knil(u16, u16) //sets A->D to nil.

    //Tables
    Global, //_G in Table(Exp::Global, target)
    Table(Box<Exp>, Box<Exp>), //name.target

    //Binary Ops
    Add(Box<Exp>, Box<Exp>),
    Sub(Box<Exp>, Box<Exp>),
    Mul(Box<Exp>, Box<Exp>),
    Div(Box<Exp>, Box<Exp>),
    Mod(Box<Exp>, Box<Exp>),
    Pow(Box<Exp>, Box<Exp>),
    Cat(Box<Exp>, Box<Exp>),

    //Unary
    Move(Box<Exp>, Box<Exp>), //assignment. move Box<Exp> into slot u16
    Unm(Box<Exp>),
    Len(Box<Exp>),

    //IST/F(C) -> A, D. if A is not empty, then it is an IST/FC op.
    IsT(Box<Exp>, Box<Exp>), //NotD = ISF.

    //Boolean
    Gt,     // >
    Gte,    // >=
    Lt,     // <
    Lte,    // <=
    Equals, // ==

    Comparison(Box<Exp>, Box<Exp>, Box<Exp>), //exp op exp
    Not(Box<Exp>),
    And(Box<Exp>, Box<Exp>),
    Or(Box<Exp>, Box<Exp>),
    
    //Branching
    UClo(u16, Box<Exp>),
    Jump(u32), //conditional, 'restrained' jumps.
    If(Box<Exp>, u16, u16), //comparison, start of scope, end of scope.
    Else(Box<Exp>, Box<Exp>),
    While(Box<Exp>, Box<Exp>),
    Repeat(Box<Exp>, Box<Exp>),

    For(Box<Exp>, Box<Exp>, Box<Exp>, Box<Exp>), //start, stop, step, Range(start->end of scope)
    IterFor(Box<Exp>, Box<Exp>), //iter call, iter block range

    //Functions
    Func(u16, Box<Exp>), //proto index, func info?
    VarArg(Box<Exp>), //var args Range(from, to)
    ParamCount(u16),
    ReturnCount(u16),
    Call(Box<Exp>, Box<Exp>, Box<Exp>), //Name, Param Range, Return Range

    //Returns
    Return(Box<Exp>),
}

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = "".to_string();

        match self {
            Exp::Empty                  => result.push_str("(empty)"),
            Exp::Redundant(v)           => result.push_str(&format!("redundant({})", v)),
            Exp::Error(v)               => result.push_str(&format!("error({})", v)),
            Exp::Range(v1, v2)          => result.push_str(&format!("{}->{}", v1, v2)),
            Exp::Goto(v)                => result.push_str(&format!("goto({})", v)),
            Exp::Var(v)                 => result.push_str(&format!("var({})", v)),
            Exp::Num(v)                 => result.push_str(&format!("num({})", v)),
            Exp::Lit(v)                 => result.push_str(&format!("lit({})", v)),
            Exp::Str(v)                 => result.push_str(&format!("str({})", v)),
            Exp::Uv(v)                  => result.push_str(&format!("uv({})", v)),
            Exp::Pri(v)                 => result.push_str(&format!("pri({})", v)),
            Exp::Global                 => result.push_str("_G"),
            Exp::Table(v1, v2)          => result.push_str(&format!("{}.{}", v1, v2)),
            Exp::Add(v1, v2)            => result.push_str(&format!("({} + {})", v1, v2)),
            Exp::Sub(v1, v2)            => result.push_str(&format!("({} - {})", v1, v2)),
            Exp::Mul(v1, v2)            => result.push_str(&format!("({} * {})", v1, v2)),
            Exp::Div(v1, v2)            => result.push_str(&format!("({} / {})", v1, v2)),
            Exp::Mod(v1, v2)            => result.push_str(&format!("({} % {})", v1, v2)),
            Exp::Pow(v1, v2)            => result.push_str(&format!("({}^{})", v1, v2)),
            Exp::Cat(v1, v2)            => result.push_str(&format!("({} .. {})", v1, v2)),
            Exp::Unm(v)                 => result.push_str(&format!("-({})", v)),
            Exp::Move(v1, v2)           => result.push_str(&format!("{} := {}", v1, v2)),
            Exp::Len(v)                 => result.push_str(&format!("len({})", v)),
            Exp::Gt                     => result.push_str(">"),
            Exp::Gte                    => result.push_str(">="),
            Exp::Lt                     => result.push_str("<"),
            Exp::Lte                    => result.push_str("<="),
            Exp::Equals                 => result.push_str("=="),
            Exp::Comparison(v1, v2, v3) => result.push_str(&format!("({} {} {})", v1, v2, v3)),
            Exp::Not(v)                 => result.push_str(&format!("not({})", v)),
            Exp::And(v1, v2)            => result.push_str(&format!("({} and {})", v1, v2)),
            Exp::Or(v1, v2)             => result.push_str(&format!("({} or {})", v1, v2)),
            Exp::UClo(v1, v2)           => result.push_str(&format!("uclo({}, {})", v1, v2)),
            Exp::Jump(v1)               => result.push_str(&format!("jmp({})", v1)),
            Exp::If(v1, v2, v3)         => result.push_str(&format!("if {} then {}:{}", v1, v2, v3)),
            Exp::Else(v1, v2)           => result.push_str(&format!("else {} range({})", v1, v2)),
            Exp::While(v1, v2)          => result.push_str(&format!("while {} range({})", v1, v2)),
            Exp::For(v1, v2, v3, v4)    => result.push_str(&format!("for start({}), stop({}), step({}), scope({})", v1, v2, v3, v4)),
            Exp::Repeat(v1, v2)         => result.push_str(&format!("repeat {} range({})", v1, v2)),
            Exp::Func(v1, v2)           => result.push_str(&format!("func(proto:{}, info:{})", v1, v2)),
            Exp::VarArg(v)              => result.push_str(&format!("varg({})", v)), 
            Exp::ParamCount(v)          => result.push_str(&format!("params({})", v)),
            Exp::ReturnCount(v)         => result.push_str(&format!("returns({})", v)),
            Exp::Call(v1, v2, v3)       => result.push_str(&format!("call({}, params({}), returns({}))", v1, v2, v3)),
            Exp::Return(v)              => result.push_str(&format!("return({})", v)),
            Exp::IsT(v1, v2)            => result.push_str(&format!("IsT({}, {})", v2, v1)),
            Exp::IterFor(v1, v2)        => result.push_str(&format!("Iter({}, {})", v1, v2)),
        }
        
        write!(f, "{}", result)
    }
}