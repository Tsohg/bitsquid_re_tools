mod comparison;
mod unary;
mod arith;
mod constant;
mod upvalue;
mod table;
mod call;
mod lj_loop;
mod ret;
mod func;

use crate::{
    dis::bytecode_instruction::Bci,
    ir::{
        blocker::Block, 
        expressions::Exp,
        translator::{
            comparison::Comparison,
            unary::Unary,
            arith::Arith,
            constant::Constant,
            upvalue::Upvalue,
            table::Table,
            call::Call,
            lj_loop::Loop,
            ret::Ret,
            func::Func,
        },
    },
};

pub struct IRBlock {
    pub expressions: Vec<Exp>,
}

pub struct IRPrototype {
    pub ir_blocks: Vec<IRBlock>,
}


pub struct Translator{}
impl Translator {
    // prototype and its blocks as vectors. Prototype<Block<Exp>>
    pub fn translate_blocks(&self, blocks: Vec<Block>) -> IRPrototype {
        let mut ir_blocks : Vec<IRBlock> = vec![];
        for block in blocks.iter() {
            ir_blocks.push(self.translate_block(&block));
        }
        IRPrototype {
            ir_blocks: ir_blocks,
        }
    }

    fn translate_block(&self, block: &Block) -> IRBlock {
        let mut expressions : Vec<Exp> = vec![];
        for bci in block.instructions.iter() {
            expressions.push(self.translate_bci(&bci));
        }
        IRBlock {
            expressions: expressions,
        }
    }

    pub fn translate_bci(&self, bci: &Bci) -> Exp {
        match bci.op {
            0..=15  => Comparison::comparison(bci),
            16..=19 => Unary::unary(bci),
            20..=36 => Arith::arith(bci),
            37..=42 => Constant::constant(bci),
            43..=48 => Upvalue::upvalue(bci),
            49      => Func::fnew(bci),
            50..=60 => Table::table(bci),
            61..=67 => Call::call(bci),
            68      => Loop::iter_jump(bci), //same as ITERJ.
            69..=72 => Ret::ret(bci),
            73..=77 => Loop::for_loop(bci),
            78..=80 => Loop::iter_loop(bci),
            81..=83 => Loop::while_loop(bci),
            84      => Exp::Jump(bci.get_jump_target()),
            85..=92 => Exp::Error("85->92, func.rs".to_string()),
            93      => Exp::Goto(bci.get_jump_target()),
            94      => Loop::iter_jump(bci),

            _ => Exp::Error(format!("translate_bci: {}", bci).to_string()),
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::{
        dis::{
            prototyper::*,
        },
        ir::{
            blocker::*,
        }
    };

    use std::fs::File;
    use std::io::Write;
    use super::*;

    fn debug_write_file(contents: &str) {
        let mut file = File::create("debug_ir.txt").unwrap();
        write!(&mut file, "{}", contents).unwrap();
    }

    fn setup() -> Vec<Block> {
        let mut ptr = Prototyper::new("dec.lua");
        let blr = Blocker{};
        let pt = ptr.next().unwrap(); //dec.ifs
        let pt = ptr.next().unwrap(); //dec.loops
        let pt = ptr.next().unwrap(); //dec.gotos
        let pt = ptr.next().unwrap(); //dec.equivgoto
        let pt = ptr.next().unwrap(); //dec.vargs
        blr.make_blocks(&pt)
    }

    #[test]
    #[ignore]
    fn test_write() {

        let t = Translator{};
        let blocks = setup();

        let mut contents = "".to_string();
        for (i, block) in blocks.iter().enumerate() {
            contents.push_str(&format!("Block: {}\n", i));
            for bci in block.instructions.iter() {
                contents.push_str(&format!("\t{}: {}\n", bci.index, t.translate_bci(bci)));
            }
            contents.push_str("\n");
        }
        debug_write_file(&contents);
    }

    #[test]
    #[ignore]
    fn test_block_translate() {
        let t = Translator{};
        let blocks = setup();
        let mut contents = "".to_string();
        let output = t.translate_block(&blocks[0]);
        contents.push_str(&format!("Block: {}\n", 0));
        for e in output.expressions {
            contents.push_str(&format!("\t{}\n", e));
        }
        debug_write_file(&contents);
    }
}

/* Comparisons:
Source Code     then    Bytecode
if x < y        then    ISGE x y
if x <= y       then    ISGT x y
if x > y        then    ISGE y x
if x >= y       then    ISGT y x

if not (x < y)  then    ISLT x y
if not (x <= y) then    ISLE x y
if not (x > y)  then    ISLT y x
if not (x >= y) then    ISLE y x

if for slots A and D (A <= D = x y), (A > D = y x)
*/