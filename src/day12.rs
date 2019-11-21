use crate::assembunny::*;
use crate::prelude::*;

pub fn pt1(program: Vec<Instruction>) -> Result<i64> {
    let mut regs = Registers::default();
    let mut program = Program::new(program)?;
    program.run_to_end(&mut regs, |_| {});
    Ok(regs[0])
}

pub fn pt2(program: Vec<Instruction>) -> Result<i64> {
    let mut regs = Registers::default();
    regs[2] = 1;
    let mut program = Program::new(program)?;
    program.run_to_end(&mut regs, |_| {});
    Ok(regs[0])
}

pub fn parse(s: &str) -> IResult<&str, Vec<Instruction>> {
    use parsers::*;
    map_res(parse_assembunny, |instrs| {
        for instr in &instrs {
            match instr {
                Instruction::Toggle(_) | Instruction::Out(_) => return Err(()),
                _ => {}
            }
        }
        Ok(instrs)
    })(s)
}

#[test]
fn day12() -> Result<()> {
    const EXAMPLE: &'static str = "\
cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a";
    let example: Vec<Instruction> = vec![
        Instruction::Copy(Value::Constant(41), Value::Register(0)),
        Instruction::Increment(0),
        Instruction::Increment(0),
        Instruction::Decrement(0),
        Instruction::JumpIfNotZero(Value::Register(0), Value::Constant(2)),
        Instruction::Decrement(0),
    ];
    test_parse!(parse, EXAMPLE => example);

    test_part!(pt1, example.clone() => 42);

    Ok(())
}
