mod merge_literals;

use crate::ir::translator::IRBlock;
use crate::dis::prototyper::Prototype;

pub trait Rule {
    fn apply(block: IRBlock) -> IRBlock;
}