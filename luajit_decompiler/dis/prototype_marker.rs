pub struct PrototypeMarker {
    
}
impl PrototypeMarker {
    ///! Returns bytecode instructions that are marked as either Unexpected, Expeceted, or IterJ.
    fn get_marked_instructions(bcis: &Vec<Bci>) -> Vec<Mark> {
        //bci[i+1] is an expected jmp.
        //bci[bci[i+1].target - 1] is an expected jmp. (aka the target of the first expected jmp - 1)
        //Any unexpected JMP/UCLO is a goto.
        //Note: This does not catch ALL gotos in original source code,
        // but that is fine as equivalent code can still be reproduced without catching them all
        // as long as they pass the above expected JMP requirements.
        let mut marks: Vec<Mark> = vec![Mark::Unexpected; bcis.len()];

        for i in 0..bcis.len() {
            if bcis[i].op < 16 {
                //comparison ops.
                marks[i + 1] = Mark::Expected;
                let target = (bcis[i + 1].get_jump_target() - 1) as usize;
                marks[target] = Mark::Expected;
            } else if marks[i] == Mark::Unexpected && bcis[i].op == 84 {
                //JMP
                let target = (bcis[i].get_jump_target()) as usize;
                if bcis[target].op == 65 {
                    //ITERC -> Expected JMPs can point to ITERC.
                    marks[i] = Mark::IterJ;
                }
            }
        }
        marks
    }

    ///! Changes bytecode instruction opcodes which are marked as Unexpected or IterJ that are also either JMP or UCLO instructions.
    fn mark_unexpected_jmps_as_goto_or_iterj(bcis: &mut Vec<Bci>, marks: Vec<Mark>) {
        for (i, m) in marks.iter().enumerate() {
            let is_jmp_or_uclo = bcis[i].op == 84 || bcis[i].op == 48;

            match *m {
                //Make unexpected JMP into a GOTO.
                Mark::Unexpected if is_jmp_or_uclo => bcis[i].op = 93,
                //Make JMP into IterJ.
                Mark::IterJ if is_jmp_or_uclo => bcis[i].op = 94,
                //Expected or conditional JMP instructions don't need changed.
                Mark::Expected => (),
                //Do nothing for the rest of the Unexpected instructions because otherwise, LOOP/FOR/FORI/etc... would be effected.
                _ => (),
            }
        }
    }
}
