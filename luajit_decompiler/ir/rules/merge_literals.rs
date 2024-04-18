use super::Rule;
use crate::ir::rules::IRBlock;
use crate::ir::expressions::Exp;

pub struct MergeLiterals {} //might want to use an iterator that accepts a rule and applies that rule @ the IR block level.
impl Rule for MergeLiterals {
    fn apply(mut block: IRBlock) -> IRBlock {
        for i in 0..block.expressions.len() {
            match &block.expressions[i] {
                Exp::Call(name, params, range) => (),
                _ => (),
            }
        }

        block
    }
}