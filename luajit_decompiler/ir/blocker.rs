// Generates basic instruction blocks from prototypes.

use std::fmt;
use std::vec::Vec;
use std::collections::BTreeSet;

use crate::dis::bytecode_instruction::Bci;
use crate::dis::prototyper::Prototype;

pub struct Block {
    pub id: usize,
    pub start_index: usize,
    pub target_index: Option<usize>,
    pub instructions: Vec<Bci>,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut bcis = String::new();
        for bci in self.instructions.iter() {
            bcis.push_str(&format!("{}\n", bci));
        }
        write!(f, "B{} (start: {}, target: {:?}): \n{}", self.id, self.start_index, self.target_index, bcis)
    }
}

pub struct Blocker{}
impl Blocker {
    /// Takes one prototype's bytecode instructions and converts it to basic blocks.
    pub fn make_blocks(&self, pt: &Prototype) -> Vec<Block> {
        let blr = Blocker{};
        let mut targets = blr.find_jump_targets(&blr.find_jump_indices(&pt), &pt);
        let mut blocks: Vec<Block> = vec![];

        let mut t1 = targets.pop_first().unwrap();
        let mut id = 0;
        loop {
            if let Some(t2) = targets.pop_first() {
                blocks.push(Block {
                    id: id,
                    start_index: t1,
                    target_index: Some(t2),
                    instructions: Vec::from(&pt.instructions[t1..t2]),
                });
                t1 = t2;
            } else {
                blocks.push(Block {
                    id: id,
                    start_index: t1,
                    target_index: None,
                    instructions: Vec::from(&pt.instructions[t1..]),
                });
                break;
            }
            id += 1;
        }
        blocks
    }

    fn find_jump_indices(&self, pt: &Prototype) -> Vec<isize> {
        let mut jump_indices: Vec<isize> = vec![];
        for (i, bci) in pt.instructions.iter().enumerate() {
            if bci.is_jump() { 
                jump_indices.push(i as isize);
            } else if bci.op < 16 { //comparison
                jump_indices.push(-(i as isize)); //mark distance 1 jumps negative.
            }
        }
        jump_indices
    }

    fn find_jump_targets(&self, jump_indices: &Vec<isize>, pt: &Prototype) -> BTreeSet<usize> {
        let mut targets: BTreeSet<usize> = BTreeSet::new();
        targets.insert(0);
        for i in jump_indices.iter() {
            if *i < 0 {
                targets.insert(2 + (-*i) as usize);
            } else {
                let jmp = &pt.instructions[*i as usize];
                targets.insert(jmp.get_jump_target() as usize);
            }
        }
        targets
    }
}

#[cfg(test)]
mod tests {
    use crate::dis::prototyper::{Prototyper, Prototype};

    use std::fs::File;
    use std::io::Write;
    use super::*;

    fn debug_write_file(blocks: &Vec<Block>, pt: &Prototype) {
        let mut file = File::create("debug_blocks.txt").unwrap();
        for block in blocks.iter() {
            writeln!(&mut file, "{}", block).unwrap();
        }
        writeln!(&mut file, "\n<<String Constants>>").unwrap();
        for (i, s) in pt.constants.strings.iter().enumerate() {
            writeln!(&mut file, "\t{}: {}", i, s).unwrap();
        }
    }

    #[test]
    fn test_find_jump_indices() {
        let mut ptr = Prototyper::new("singleif.ljc");
        let pt = ptr.next().unwrap();
        let blr = Blocker{};
        let indices = blr.find_jump_indices(&pt);
        assert!(indices.len() == 6, "Expected: {}, actual: {}", 6, indices.len());
        assert!(indices[0] == -2); //ISGE
        assert!(indices[1] == 3); //JMP
        assert!(indices[2] == -9); //ISGE
        assert!(indices[3] == 10); //JMP
        assert!(indices[4] == -16); //ISGE
        assert!(indices[5] == 17); //JMP
    }

    #[test]
    fn test_find_jump_targets() {
        let mut ptr = Prototyper::new("singleif.ljc");
        let pt = ptr.next().unwrap();
        let blr = Blocker{};
        let targets = blr.find_jump_targets(&blr.find_jump_indices(&pt), &pt);
        let expected_targets: BTreeSet<usize> = [0, 4, 11, 18, 21].iter().cloned().collect();
        assert!(expected_targets.difference(&targets).count() == 0, "\nexpected: {:?}\nfound: {:?}\n", expected_targets, targets);
    }

    #[test]
    fn test_make_blocks() {
        let mut ptr = Prototyper::new("singleif.ljc");
        let pt = ptr.next().unwrap();
        let blr = Blocker{};
        let blocks = blr.make_blocks(&pt);
        //debug_write_file(&blocks, &pt);
        assert!(blocks.len() == 5);
        assert!(blocks[0].instructions.len() == 4);
        assert!(blocks[1].instructions.len() == 7);
        assert!(blocks[2].instructions.len() == 7);
        assert!(blocks[3].instructions.len() == 3);
        assert!(blocks[4].instructions.len() == 1);

        assert!(blocks[0].instructions[..] == pt.instructions[0..4]);
        assert!(blocks[1].instructions[..] == pt.instructions[4..11]);
        assert!(blocks[2].instructions[..] == pt.instructions[11..18]);
        assert!(blocks[3].instructions[..] == pt.instructions[18..21]);
        assert!(blocks[4].instructions[..] == pt.instructions[21..]);
    }

    #[test]
    fn debug_write_blocks() {
        let mut ptr = Prototyper::new("dec.lua");
        //let mut ptr = Prototyper::new("beam_system_client.lua"); //11 prototypes.
        let pt = ptr.next().unwrap(); //dec.ifs
        let pt = ptr.next().unwrap(); //dec.loops
        let pt = ptr.next().unwrap(); //dec.gotos
        let pt = ptr.next().unwrap(); //dec.equivgoto
        let pt = ptr.next().unwrap(); //dec.vargs
        //let pt = ptr.next().unwrap(); //file

        //beam_system_client
        /*let pt = ptr.next();
        let pt = ptr.next();
        let pt = ptr.next();*/ //overflow in read_uleb again...
        
        let blr = Blocker{};
        let blocks = blr.make_blocks(&pt);
        debug_write_file(&blocks, &pt);
    }
}