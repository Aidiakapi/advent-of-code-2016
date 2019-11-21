use crate::assembunny::*;
use crate::prelude::*;

pub fn pt(input: Vec<Instruction>) -> Result<i64> {
    let mut prog = Program::new(input)?;
    let mut start_idx = -1i64;
    let mut state: HashSet<(i64, Registers, bool)> = HashSet::new();
    'outer: loop {
        prog.instruction_ptr = 0;
        start_idx += 1;
        let mut expect_high = false;
        let mut regs = Registers::default();
        regs[0] = start_idx;

        state.clear();
        state.insert((prog.instruction_ptr, regs, expect_high));
        let mut has_false_output = false;
        while prog.run_one(&mut regs, |value| {
            let is_correct = match value {
                0 => !expect_high,
                1 => expect_high,
                _ => false,
            };
            if !is_correct {
                has_false_output = true;
            }
            expect_high = !expect_high;
        }) {
            if has_false_output {
                continue 'outer;
            }
            if !state.insert((prog.instruction_ptr, regs, expect_high)) {
                break 'outer Ok(start_idx);
            }
        }
    }
}

pub fn parse(s: &str) -> IResult<&str, Vec<Instruction>> {
    use parsers::*;
    map_res(parse_assembunny, |instrs| {
        for instr in &instrs {
            match instr {
                Instruction::Toggle(_) => return Err(()),
                _ => {}
            }
        }
        Ok(instrs)
    })(s)
}
